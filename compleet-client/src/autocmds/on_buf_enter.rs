use bindings::opinionated::Buffer;
use mlua::prelude::{Lua, LuaFunction, LuaResult};

use crate::state::State;
use crate::utils;

/// Called on every `BufEnter` event.
pub fn on_buf_enter(
    lua: &Lua,
    state: &mut State,
    on_insert_leave: LuaFunction,
    on_cursor_moved_i: LuaFunction,
    on_bytes: LuaFunction,
) -> LuaResult<()> {
    let buffer = Buffer::get_current(lua)?;

    // Don't attach if:
    //
    // 1. the buffer is already attached;
    //
    // 2. the `modifiable` option is turned off. This should catch a large
    //    number of buffers we'd like to ignore like netwr, startify, terminal,
    //    help, etc;
    //
    // 3. there aren't any enabled sources for this buffer.
    if state.is_buffer_attached(&buffer)
        || !buffer.get_option(lua, "modifiable")?
        || !state
            .channel
            .as_mut()
            .expect("channel already created")
            .should_attach(lua, &buffer)?
    {
        return Ok(());
    }

    if !buffer.on_bytes(lua, on_bytes)? {
        // Echo an error if for some reason we couldn't attach to the buffer.
        utils::echoerr(lua, "Couldn't attach to buffer")?;
    } else {
        // Add two buffer-local autocommands on this buffer.
        state.augroup.set_local(
            lua,
            buffer.bufnr,
            vec![
                ("CursorMovedI", on_cursor_moved_i),
                ("InsertLeave", on_insert_leave),
            ],
        )?;

        state.attach_buffer(buffer);
    };

    Ok(())
}
