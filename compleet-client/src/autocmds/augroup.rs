use std::{cell::RefCell, rc::Rc};

use mlua::prelude::{Lua, LuaFunction, LuaRegistryKey, LuaResult, LuaValue};

use crate::channel;
use crate::state::State;
use crate::ui;
use crate::{bindings::api, constants::AUGROUP_NAME, ui::Buffer};

#[derive(Debug, Default)]
pub struct Augroup {
    /// The augroup id returned by `vim.api.nvim_create_autocmd`.
    id: Option<u16>,

    /// The registry key of the Lua function called on `BufEnter` to try to
    /// attach to the buffer.
    on_buf_enter_key: Option<LuaRegistryKey>,
}

impl Augroup {
    pub fn new(lua: &Lua, state: &Rc<RefCell<State>>) -> LuaResult<Self> {
        let cloned = state.clone();
        let on_insert_leave_key = lua.create_registry_value(
            // Called on every `InsertLeave` event in attached buffers.
            lua.create_function(move |lua, ()| {
                let state = &mut cloned.borrow_mut();
                state.channel.as_mut().unwrap().stop_tasks();
                ui::cleanup(lua, &mut state.ui)?;
                state.completions.clear();
                Ok(())
            })?,
        )?;

        let cloned = state.clone();
        let on_cursor_moved_i_key = lua.create_registry_value(
            // Called on every `CursorMovedI` event in attached buffers.
            lua.create_function(move |lua, ()| {
                let state = &mut cloned.borrow_mut();
                // If the cursor was moved right after a call to `on_bytes` we
                // reset `did_on_bytes` to `false` and ignore the event.
                if state.did_on_bytes {
                    state.did_on_bytes = false;
                }
                // If not we send a notification to the server to stop all
                // running tasks and cleanup the UI.
                else {
                    state.channel.as_mut().unwrap().stop_tasks();
                    ui::cleanup(lua, &mut state.ui)?;
                    state.completions.clear();
                }
                Ok(())
            })?,
        )?;

        let cloned = state.clone();
        let on_bytes_key = lua.create_registry_value(
            // Called on every byte change in attached buffers.
            lua.create_function(
                move |lua,
                      (
                    _,
                    bufnr,
                    changedtick,
                    start_row,
                    start_col,
                    _,
                    rows_deleted,
                    _,
                    bytes_deleted,
                    rows_added,
                    _,
                    bytes_added,
                ): (
                    String,
                    _,
                    _,
                    _,
                    _,
                    u16,
                    _,
                    u16,
                    _,
                    _,
                    u16,
                    _,
                )| {
                    channel::on_bytes(
                        lua,
                        &mut cloned.borrow_mut(),
                        bufnr,
                        changedtick,
                        start_row,
                        start_col,
                        rows_deleted,
                        bytes_deleted,
                        rows_added,
                        bytes_added,
                    )
                },
            )?,
        )?;

        let clone = state.clone();
        // Called on every `BufEnter` event to check if we should attach to
        // that buffer.
        let on_buf_enter = lua.create_function(move |lua, ()| {
            super::on_buf_enter(
                lua,
                &mut clone.borrow_mut(),
                lua.registry_value::<LuaFunction>(&on_insert_leave_key)?,
                lua.registry_value::<LuaFunction>(&on_cursor_moved_i_key)?,
                lua.registry_value::<LuaFunction>(&on_bytes_key)?,
            )
        })?;

        Ok(Self {
            on_buf_enter_key: Some(lua.create_registry_value(on_buf_enter)?),
            ..Default::default()
        })
    }
}

impl Augroup {
    /// TODO: docs
    pub fn clear_local(&self, lua: &Lua, buffer: &Buffer) -> LuaResult<()> {
        api::clear_autocmds(
            lua,
            lua.create_table_from([("buffer", buffer.number)])?,
        )
    }

    /// TODO: docs
    pub fn is_set(&self) -> bool {
        self.id.is_some()
    }

    /// Creates a new augroup, adding a global autocommand on the `BufEnter`
    /// event to attach to buffers. Should only be called if the augroup is not
    /// set.
    pub fn set(&mut self, lua: &Lua) -> LuaResult<()> {
        let id = api::create_augroup(
            lua,
            AUGROUP_NAME,
            lua.create_table_from([("clear", true)])?,
        )?;

        let on_buf_enter = lua.registry_value::<LuaFunction>(
            self.on_buf_enter_key.as_ref().expect("augroup already setup"),
        )?;

        api::create_autocmd(
            lua,
            ["BufEnter"],
            lua.create_table_from([
                ("group", LuaValue::Integer(id as i64)),
                ("callback", LuaValue::Function(on_buf_enter)),
            ])?,
        )?;

        self.id = Some(id);

        Ok(())
    }

    /// Add new buffer-local autocommands to the group. Should only be called
    /// if the augroup is set.
    pub fn set_local(
        &self,
        lua: &Lua,
        buffer: &Buffer,
        events: Vec<(&'static str, LuaFunction)>,
    ) -> LuaResult<()> {
        let id = self.id.expect("augroup is set");

        for (event, callback) in events {
            let opts = lua.create_table_from([
                ("group", LuaValue::Integer(id as i64)),
                ("callback", LuaValue::Function(callback)),
                ("buffer", LuaValue::Integer(buffer.number as i64)),
            ])?;

            api::create_autocmd(lua, vec![event], opts)?;
        }

        Ok(())
    }

    /// TODO: docs
    pub fn unset(&mut self, lua: &Lua) -> LuaResult<()> {
        if let Some(id) = self.id {
            api::del_augroup_by_id(lua, id)?;
            self.id = None;
        }

        Ok(())
    }
}
