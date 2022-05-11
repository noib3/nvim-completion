use bindings::opinionated::{Buffer, OnBytesHook};
use mlua::prelude::{Lua, LuaFunction};

use crate::client::Client;
use crate::messages;

/// Called every time the user enters a buffer.
pub fn on_buf_enter(
    lua: &Lua,
    client: &mut Client,
    on_insert_leave: LuaFunction,
    on_cursor_moved_i: LuaFunction,
    on_bytes: LuaFunction,
) -> mlua::Result<()> {
    let buf = Buffer::get_current(lua)?;
    let already_seen = client.entered_buffer(&buf);

    if already_seen
        || !buf.is_modifiable(lua)?
        || !client.has_enabled_sources(lua, &buf)?
    {
        return Ok(());
    }

    match buf.attach_on_bytes(lua, on_bytes) {
        Ok(()) => {
            let autocmds = &[
                ("CursorMovedI", on_cursor_moved_i),
                ("InsertLeave", on_insert_leave),
            ];

            client.register_autocommands(lua, autocmds, Some(&buf))?;
            client.attach_buffer(buf)
        },

        Err(reason) => {
            messages::echoerr!(lua, "Couldn't attach to buffer: {reason}",)?
        },
    }

    Ok(())
}
