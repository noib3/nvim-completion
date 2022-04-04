use compleet::rpc::message::RpcNotification;
use mlua::{prelude::LuaResult, Lua};

use crate::bindings::nvim;
use crate::state::State;

pub fn handle_notify(
    lua: &Lua,
    _state: &mut State,
    notification: RpcNotification,
) -> LuaResult<()> {
    nvim::print(
        lua,
        format!("Got a notification with method: {}", notification.method),
    )?;

    Ok(())
}
