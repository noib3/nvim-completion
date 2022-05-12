use bindings::opinionated::Buffer;
use mlua::Lua;

use crate::client::Client;
use crate::messages;

pub fn attach_all(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    if client.is_completion_on() {
        self::attach_current(lua, client)?;
        return Ok(());
    }

    client.attach_all_buffers(lua)?;
    messages::echoinfo!(lua, "Started completion in all buffers")
}

pub fn attach_current(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    let buf = Buffer::get_current(lua)?;

    if let Err(err) = client.attach_buffer(lua, buf) {
        messages::echoerr!(lua, "Couldn't attach to buffer: {err}")?;
        return Ok(());
    }

    messages::echoinfo!(lua, "Started completion in buffer {current}")
}
