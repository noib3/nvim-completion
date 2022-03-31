use std::sync::Arc;

use mlua::prelude::{Lua, LuaRegistryKey, LuaResult};
use neovim::{Api, Neovim};
use parking_lot::Mutex;

use crate::completion;
use crate::state::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<(u32, LuaRegistryKey)> {
    // Called on every `InsertLeave` event of attached buffers.
    let _state = state.clone();
    let insert_leave = move |lua, ()| {
        let state = &mut _state.lock();
        let api = Neovim::new(lua)?.api;
        // Abort all pending tasks.
        state.handles.iter().for_each(|handle| handle.abort());
        state.handles.clear();
        // Cleanup the UI.
        state.ui.cleanup(&api)
    };

    // Called on every `CursorMovedI` event of attached buffers.
    let _state = state.clone();
    let cursor_moved_i = move |lua, ()| {
        let state = &mut _state.lock();
        // If the cursor was moved right after a call to `on_bytes` we
        // reset `did_on_bytes` to `false` and ignore the event.
        if state.did_on_bytes {
            state.did_on_bytes = false;
        }
        // If not we abort all pending tasks and cleanup the UI.
        else {
            state.handles.iter().for_each(|handle| handle.abort());
            state.handles.clear();
            let api = Neovim::new(lua)?.api;
            state.ui.cleanup(&api)?;
        }
        Ok(())
    };

    // Called every time a byte in an attached buffer is changed.
    let _state = state.clone();
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
            completion::on_bytes(
                lua,
                &mut _state.lock(),
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
    let _state = state.clone();
    let try_buf_attach = lua.create_function(move |lua, ()| {
        super::try_buf_attach(
            lua,
            &mut _state.lock(),
            lua.create_function(insert_leave.clone())?,
            lua.create_function(cursor_moved_i.clone())?,
            lua.create_function(on_bytes.clone())?,
        )
    })?;

    // Create the `Compleet` augroup which will namespace all the other
    // autocmds.
    let augroup_id = api.create_augroup(
        "Compleet",
        lua.create_table_from([("clear", true)])?,
    )?;

    // Register an autocmd for the `BufEnter` event to try to attach to the new
    // buffer.
    let opts = lua.create_table_with_capacity(0, 2)?;
    opts.set("group", augroup_id)?;
    opts.set("callback", try_buf_attach.clone())?;
    api.create_autocmd(&["BufEnter"], opts.clone())?;

    Ok((augroup_id, lua.create_registry_value(try_buf_attach)?))
}
