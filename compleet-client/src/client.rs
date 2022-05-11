use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bindings::opinionated::Buffer;
use mlua::Lua;
use sources::prelude::{Completions, Cursor};

use crate::{autocmds::Augroup, channel::Channel, settings::Settings, ui::Ui};

// TODO: refactor
// rename channel to bridge.
// make variables private
// expose functionality via functions

/// Holds state about the state of the compleet client.
#[derive(Default)]
pub struct Client {
    /// The currently attached buffers.
    pub attached_buffers: HashMap<u16, Arc<Buffer>>,

    /// The augroup namespacing all the autocommands.
    pub augroup: Augroup,

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    buffers_to_be_detached: Vec<u16>,

    /// TODO: docs
    pub changedtick_last_seen: u32,

    /// TODO: docs
    pub changedtick_last_update: u32,

    /// A channel used to communicate w/ the tokio threadpool where the
    /// completion results are computed.
    pub channel: Option<Channel>,

    /// The currently available completion items.
    pub completions: Completions,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// Set to `true` right after `on_bytes` gets called.
    pub did_on_bytes: bool,

    /// TODO: docs.
    pub ignore_next_on_bytes: bool,

    /// Whether the setup function has ever been called.
    pub did_setup: bool,

    /// TODO: docs
    pub matched_bytes: usize,

    /// The current settings.
    pub settings: Settings,

    /// The current state of the UI.
    pub ui: Ui,

    /// The set of buffer numbers the client has seen. A buffer is seen when
    /// after it triggers the `BufEnter` autocommand for the first time.
    seen_bufnrs: HashSet<u16>,

    /// Whether the next `CursorMovedI` event will be ignored.
    skip_next_cursor_moved_i: bool,
}

impl Client {
    /// TODO: docs
    pub fn attach_buffer(&mut self, buffer: Buffer) {
        self.attached_buffers.insert(buffer.bufnr, Arc::new(buffer));
    }

    /// TODO: docs
    pub fn cancel_detach_all(&mut self) {
        self.buffers_to_be_detached.clear()
    }

    /// TODO: docs
    pub fn cancel_detach_buffer(&mut self, buffer: &Buffer) {
        self.buffers_to_be_detached.retain(|&b| b != buffer.bufnr);
    }

    /// TODO: docs
    pub fn detach_all_buffers(&mut self, lua: &Lua) -> mlua::Result<()> {
        let attached = self.attached_buffers.drain().map(|(bufnr, _)| bufnr);
        self.buffers_to_be_detached.extend(attached);
        self.augroup.unset(lua)?;
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
        self.augroup.clear_local(lua, buffer.bufnr)?;
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
        self.augroup.is_set()
    }

    /// TODO: docs
    pub fn should_detach(&mut self, bufnr: u16) -> bool {
        if !self.buffers_to_be_detached.contains(&bufnr) {
            return false;
        }
        self.buffers_to_be_detached.retain(|&num| bufnr != num);
        true
    }

    /// TODO: docs
    pub fn _get_buffer(&self, bufnr: u16) -> Option<Arc<Buffer>> {
        self.attached_buffers.get(&bufnr).map(|buf| buf.clone())
    }

    /// Notifies the client that the user entered a buffer. Returns `true` if
    /// the client had already seen that buffer.
    pub fn entered_buffer(&mut self, buf: &Buffer) -> bool {
        self.seen_bufnrs.insert(buf.bufnr)
    }

    /// TODO: docs
    pub fn should_attach(
        &mut self,
        lua: &Lua,
        buf: &Buffer,
    ) -> mlua::Result<bool> {
        self.channel
            .as_mut()
            .expect("bridge already created")
            .should_attach(lua, buf)
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
