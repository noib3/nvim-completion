use bindings::opinionated::Buffer;
use mlua::Lua;

use crate::client::Client;
use crate::messages;

pub fn detach_all(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    if !client.is_completion_on() {
        messages::echoerr!(lua, "Completion is already off")?;
        return Ok(());
    }

    client.detach_all_buffers(lua);
    messages::echoinfo!(lua, "Stopped completion in all buffers")
}

pub fn detach_current(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    let current = Buffer::get_current(lua)?;

    if !client.is_buffer_attached(&current) {
        messages::echoerr!(lua, "Completion is already off in {current}")?;
        return Ok(());
    }

    client.detach_buffer(lua, &current)?;
    messages::echoinfo!(lua, "Stopped completion in buffer {current}")
}
