use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bindings::api;
use bindings::opinionated::buffer::{Buffer, LuaFn, OnBytesSignature};
use mlua::Lua;
use sources::prelude::{Completions, Cursor};

use super::{AttachError, LateInit};
use crate::channel::Channel;
use crate::settings::Settings;
use crate::ui::Ui;

const AUGROUP_NAME: &str = "Compleet";

#[derive(Default)]
pub struct Client {
    /// The currently attached buffers.
    pub attached_buffers: HashMap<u32, Arc<Buffer>>,

    augroup_id: Option<u32>,
    buffers_to_be_detached: Vec<u32>,

    pub changedtick_last_seen: u32,

    pub changedtick_last_update: u32,

    /// A channel used to communicate w/ the tokio threadpool where the
    /// completion results are computed.
    pub channel: LateInit<RefCell<Channel>>,

    /// The currently available completion items.
    pub completions: Completions,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// Set to `true` right after `on_bytes` gets called.
    pub did_on_bytes: bool,

    /// TODO: docs.
    pub ignore_next_on_bytes: bool,

    /// Whether the setup function has ever been called.
    did_setup: bool,

    /// TODO: docs
    pub matched_bytes: usize,

    /// The current settings.
    pub settings: Settings,

    /// The current state of the UI.
    pub ui: Ui,

    /// The set of buffer numbers the client has seen. A buffer is seen when
    /// after it triggers the `BufEnter` autocommand for the first time.
    seen_bufnrs: HashSet<u32>,

    /// Whether the next `CursorMovedI` event will be ignored.
    skip_next_cursor_moved_i: bool,

    on_cursor_moved_i: LateInit<LuaFn<(), ()>>,
    on_insert_leave: LateInit<LuaFn<(), ()>>,
    on_buf_enter: LateInit<LuaFn<(), ()>>,
    on_bytes: LateInit<LuaFn<OnBytesSignature, Option<bool>>>,
}

// Public impl block.
impl Client {
    pub fn did_setup(&self) -> bool {
        self.did_setup
    }

    /// TODO: docs
    pub fn attach_buffer(
        &mut self,
        lua: &Lua,
        buf: Buffer,
    ) -> Result<(), AttachError> {
        if self.is_buffer_attached(&buf) {
            return Err(AttachError::AlreadyAttached(buf));
        }

        if !buf.is_modifiable(lua)? {
            return Err(AttachError::NotModifiable(buf));
        }

        if !self.has_enabled_sources(lua, &buf)? {
            return Err(AttachError::NoSourcesEnabled(buf));
        }

        if buf.attach_on_bytes(lua, *self.on_bytes).is_err() {
            return Err(AttachError::AttachFailed);
        }

        self.set_buflocal_autocmds(lua, &buf)?;
        self.buffers_to_be_detached.retain(|&b| b != buf.bufnr);
        self.attached_buffers.insert(buf.bufnr, Arc::new(buf));

        Ok(())
    }

    pub fn attach_all_buffers(
        &mut self,
        lua: &Lua,
    ) -> Result<(), AttachError> {
        self.buffers_to_be_detached.clear();
        self.set_augroup(lua)?;
        self.attach_buffer(lua, Buffer::get_current(lua)?)
    }

    /// TODO: docs
    pub fn setup(
        &mut self,
        lua: &Lua,
        (on_insert_leave, on_cursor_moved_i, on_buf_enter, on_bytes): (
            LuaFn<(), ()>,
            LuaFn<(), ()>,
            LuaFn<(), ()>,
            LuaFn<OnBytesSignature, Option<bool>>,
        ),
        settings: Settings,
    ) -> mlua::Result<()> {
        self.on_insert_leave = LateInit::set(on_insert_leave);
        self.on_cursor_moved_i = LateInit::set(on_cursor_moved_i);
        self.on_buf_enter = LateInit::set(on_buf_enter);
        self.on_bytes = LateInit::set(on_bytes);

        self.settings = settings;
        self.did_setup = true;

        self.set_augroup(lua)?;

        Ok(())
    }

    /// TODO: docs
    pub fn detach_all_buffers(&mut self, lua: &Lua) -> mlua::Result<()> {
        let attached = self.attached_buffers.drain().map(|(bufnr, _)| bufnr);
        self.buffers_to_be_detached.extend(attached);
        self.clear_augroup(lua)?;
        self.cleanup_ui(lua)?;
        self.clear_completions();
        Ok(())
    }

    /// TODO: docs
    pub fn detach_buffer(
        &mut self,
        lua: &Lua,
        buffer: &Buffer,
    ) -> mlua::Result<()> {
        self.attached_buffers.remove(&buffer.bufnr);
        self.buffers_to_be_detached.push(buffer.bufnr);
        self.clear_buflocal_autocmds(lua, &buffer)?;
        self.cleanup_ui(lua);
        self.clear_completions();
        Ok(())
    }

    /// Cleans up all the currently displayed UI elements from the screen.
    pub fn cleanup_ui(&mut self, lua: &Lua) -> mlua::Result<()> {
        todo!()
    }

    /// TODO: docs
    pub fn clear_completions(&mut self) {
        self.completions.clear()
    }

    /// TODO: docs
    pub fn is_buffer_attached(&self, buffer: &Buffer) -> bool {
        self.attached_buffers.get(&buffer.bufnr).is_some()
    }

    /// TODO: docs
    pub fn is_completion_on(&self) -> bool {
        self.augroup_id.is_some()
    }

    /// TODO: docs
    pub fn should_detach(&mut self, bufnr: u32) -> bool {
        if !self.buffers_to_be_detached.contains(&bufnr) {
            return false;
        }
        self.buffers_to_be_detached.retain(|&num| bufnr != num);
        true
    }

    /// TODO: docs
    pub fn _get_buffer(&self, bufnr: u32) -> Option<Arc<Buffer>> {
        self.attached_buffers.get(&bufnr).map(|buf| buf.clone())
    }

    /// Notifies the client that the user entered a buffer. Returns `true` if
    /// the client had already seen that buffer.
    pub fn entered_buffer(&mut self, buf: &Buffer) -> bool {
        !self.seen_bufnrs.insert(buf.bufnr)
    }

    /// TODO: docs
    pub fn should_attach(
        &mut self,
        lua: &Lua,
        buf: &Buffer,
    ) -> mlua::Result<bool> {
        self.channel.get_mut().should_attach(lua, buf)
    }

    /// TODO: docs
    pub fn register_autocommands(
        &self,
        lua: &Lua,
        commands: &[(&'static str, mlua::Function)],
        buf: Option<&Buffer>,
    ) -> mlua::Result<()> {
        todo!()
    }

    /// TODO: docs
    pub fn should_skip_next_cursor_moved_i(&mut self) -> bool {
        if self.skip_next_cursor_moved_i {
            self.skip_next_cursor_moved_i = false;
            return true;
        }
        false
    }

    /// TODO: docs
    pub fn stop_fetching_completions(&mut self) {
        todo!()
    }

    /// Returns `true` if there's at least one completion source enabled for
    /// the buffer.
    pub fn has_enabled_sources(
        &self,
        lua: &Lua,
        buf: &Buffer,
    ) -> mlua::Result<bool> {
        todo!()
    }
}

// Private impl block.
impl Client {
    /// Creates the `Compleet` augroup, adding a global autocommand on the
    /// `BufEnter` event.
    fn set_augroup(&mut self, lua: &Lua) -> mlua::Result<u32> {
        let opts = lua.create_table_from([("clear", true)])?;
        let id = api::create_augroup(lua, AUGROUP_NAME, opts)?.into();

        let opts = lua.create_table_with_capacity(0, 2)?;
        opts.set("group", id)?;
        opts.set("callback", lua.create_function(*self.on_buf_enter)?)?;
        api::create_autocmd(lua, ["BufEnter"], opts)?;

        self.augroup_id = Some(id);

        Ok(id)
    }

    /// Deletes the `Compleet` augroup and all its autocommands.
    fn clear_augroup(&mut self, lua: &Lua) -> mlua::Result<()> {
        if let Some(id) = self.augroup_id {
            api::del_augroup_by_id(lua, id.try_into().unwrap())?;
            self.augroup_id = None;
        }

        Ok(())
    }

    /// Sets the two buffer-local autocommands on the `CursorMovedI` and
    /// `InsertLeave` events, creating the `Compleet` augroup if it's not
    /// already set.
    fn set_buflocal_autocmds(
        &mut self,
        lua: &Lua,
        buf: &Buffer,
    ) -> mlua::Result<()> {
        let id =
            self.augroup_id.unwrap_or_else(|| self.set_augroup(lua).unwrap());

        let opts = lua.create_table_with_capacity(0, 3)?;
        opts.set("group", id)?;
        opts.set("buffer", buf.bufnr)?;

        opts.set("callback", lua.create_function(*self.on_cursor_moved_i)?)?;
        api::create_autocmd(lua, vec!["CursorMovedI"], opts.clone())?;

        opts.set("callback", lua.create_function(*self.on_insert_leave)?)?;
        api::create_autocmd(lua, vec!["InsertLeave"], opts)?;

        Ok(())
    }

    /// Clears all the buffer-local autocommands in the `Compleet` augroup set
    /// on a buffer.
    fn clear_buflocal_autocmds(
        &self,
        lua: &Lua,
        buf: &Buffer,
    ) -> mlua::Result<()> {
        if let Some(id) = self.augroup_id {
            let opts = lua.create_table_from([
                ("group", id),
                ("buffer", buf.bufnr.into()),
            ])?;
            api::clear_autocmds(lua, opts)?;
        }

        Ok(())
    }
}
