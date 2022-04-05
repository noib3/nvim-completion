use std::{cell::RefCell, rc::Rc};

use compleet::api::incoming::Notification;
use mlua::prelude::{Lua, LuaRegistryKey, LuaResult};

use crate::bindings::api;
use crate::channel;
use crate::state::State;
use crate::ui;

pub fn setup(
    lua: &Lua,
    state: &Rc<RefCell<State>>,
) -> LuaResult<(u32, LuaRegistryKey)> {
    // Called on every `InsertLeave` event of attached buffers.
    let cloned = state.clone();
    let insert_leave = move |lua, ()| {
        let mut borrowed = cloned.borrow_mut();
        // Send a notification to the server to stop all running tasks, then
        // cleanup the UI.
        borrowed
            .channel
            .as_ref()
            .unwrap()
            .notify(lua, Notification::StopTasks)?;
        ui::cleanup(lua, &mut borrowed.ui.as_mut().unwrap())?;
        Ok(())
    };

    // Called on every `CursorMovedI` event of attached buffers.
    let cloned = state.clone();
    let cursor_moved_i = move |lua, ()| {
        let mut borrowed = cloned.borrow_mut();
        // If the cursor was moved right after a call to `on_bytes` we
        // reset `did_on_bytes` to `false` and ignore the event.
        if borrowed.did_on_bytes {
            borrowed.did_on_bytes = false;
        }
        // If not we send a notification to the server to stop any running
        // tasks and cleanup the UI.
        else {
            borrowed
                .channel
                .as_ref()
                .unwrap()
                .notify(lua, Notification::StopTasks)?;
            ui::cleanup(lua, &mut borrowed.ui.as_mut().unwrap())?;
        }
        Ok(())
    };

    // Called every time a byte in an attached buffer is changed.
    let cloned = state.clone();
    let on_bytes =
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
        };

    // Called on every `BufEnter` event.
    let clone = state.clone();
    let try_buf_attach = lua.create_function(move |lua, ()| {
        super::try_buf_attach(
            lua,
            &mut clone.borrow_mut(),
            lua.create_function(insert_leave.clone())?,
            lua.create_function(cursor_moved_i.clone())?,
            lua.create_function(on_bytes.clone())?,
        )?;
        Ok(())
    })?;

    // Create the `Compleet` augroup which will namespace all the other
    // autocmds.
    let augroup_id = api::create_augroup(
        lua,
        "Compleet",
        lua.create_table_from([("clear", true)])?,
    )?;

    // Register an autocmd for the `BufEnter` event to try to attach to the new
    // buffer.
    let opts = lua.create_table_with_capacity(0, 2)?;
    opts.set("group", augroup_id)?;
    opts.set("callback", try_buf_attach.clone())?;
    api::create_autocmd(lua, &["BufEnter"], opts.clone())?;

    Ok((augroup_id, lua.create_registry_value(try_buf_attach)?))
}
