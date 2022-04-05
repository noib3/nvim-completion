use std::{cell::RefCell, rc::Rc};

use compleet::api::incoming::Notification;
use mlua::prelude::{Lua, LuaFunction, LuaRegistryKey, LuaResult};

use super::Augroup;
use crate::channel;
use crate::state::State;
use crate::ui;

/// Creates a new augroup and registers a callback on the `BufEnter` event,
/// returning the id of the augroup and the Lua registry key of the callback.
pub fn setup(
    lua: &Lua,
    state: &Rc<RefCell<State>>,
) -> LuaResult<(Augroup, LuaRegistryKey)> {
    // Called on every `InsertLeave` event in attached buffers.
    let cloned = state.clone();
    let on_insert_leave_key =
        lua.create_registry_value(lua.create_function(move |lua, ()| {
            let state = &mut cloned.borrow_mut();
            // Send a notification to the server to stop all running tasks,
            // then cleanup the UI.
            state.channel.notify(lua, Notification::StopTasks)?;
            ui::cleanup(lua, &mut state.ui)?;
            Ok(())
        })?)?;

    // Called on every `CursorMovedI` event in attached buffers.
    let cloned = state.clone();
    let on_cursor_moved_i_key =
        lua.create_registry_value(lua.create_function(move |lua, ()| {
            let state = &mut cloned.borrow_mut();
            // If the cursor was moved right after a call to `on_bytes` we
            // reset `did_on_bytes` to `false` and ignore the event.
            if state.did_on_bytes {
                state.did_on_bytes = false;
            }
            // If not we send a notification to the server to stop all running
            // tasks and cleanup the UI.
            else {
                state.channel.notify(lua, Notification::StopTasks)?;
                ui::cleanup(lua, &mut state.ui)?;
            }
            Ok(())
        })?)?;

    // Called on every byte change in attached buffers.
    let cloned = state.clone();
    let on_bytes_key = lua.create_registry_value(lua.create_function(
        move |lua,
              (
            _,
            bufnr,
            _,
            start_row,
            start_col,
            _,
            rows_deleted,
            _,
            bytes_deleted,
            rows_added,
            _,
            bytes_added,
        ): (String, _, u32, _, _, u32, _, u32, _, _, u32, _)| {
            channel::on_bytes(
                lua,
                &mut cloned.borrow_mut(),
                bufnr,
                start_row,
                start_col,
                rows_deleted,
                bytes_deleted,
                rows_added,
                bytes_added,
            )
        },
    )?)?;

    // Called on every `BufEnter` event to check if we should attach to that
    // buffer.
    let clone = state.clone();
    let on_buf_enter = lua.create_function(move |lua, ()| {
        super::on_buf_enter(
            lua,
            &mut clone.borrow_mut(),
            lua.registry_value::<LuaFunction>(&on_insert_leave_key)?,
            lua.registry_value::<LuaFunction>(&on_cursor_moved_i_key)?,
            lua.registry_value::<LuaFunction>(&on_bytes_key)?,
        )?;
        Ok(())
    })?;

    // Create the augroup which will namespace all the autocmds.
    let mut augroup = Augroup::new(lua)?;

    // Register a global autocmd on the `BufEnter` event.
    augroup.add_autocmds(
        lua,
        None,
        vec![("BufEnter", on_buf_enter.clone())],
    )?;

    Ok((augroup, lua.create_registry_value(on_buf_enter)?))
}
