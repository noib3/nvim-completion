use bindings::opinionated::buffer::Buffer;
use mlua::Lua;

use crate::client::{AttachError, Client};
use crate::messages;

/// Called every time the user enters a buffer.
pub fn on_buf_enter(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    let buf = Buffer::get_current(lua)?;

    if client.entered_buffer(&buf) {
        return Ok(());
    }

    if let Err(err) = client.attach_buffer(lua, buf) {
        use AttachError::*;
        match err {
            AlreadyAttached(_) | NotModifiable(_) | NoSourcesEnabled(_) => {},
            _ => messages::echoerr!(lua, "Couldn't attach to buffer: {err}")?,
        };
    }

    Ok(())
}
