use compleet::{api::outgoing::Notification, rpc::RpcMessage};
use mlua::prelude::{Lua, LuaResult};

use crate::state::State;
use crate::utils;

/// Called when ...
pub fn on_stderr(
    lua: &Lua,
    state: &mut State,
    bytes: Vec<u8>,
) -> LuaResult<()> {
    let _ciao = bytes.clone();

    // TODO: spawn a new oneshot channel to do the conversion.
    // std::thread::sleep(std::time::Duration::from_secs(3));

    match RpcMessage::try_from(bytes) {
        Ok(message) => match message {
            RpcMessage::Request { .. } => {},

            RpcMessage::Response { .. } => {},

            RpcMessage::Notification { method, params } => {
                match Notification::try_from((method, params)) {
                    Ok(ntf) => super::handle_notify(lua, state, ntf)?,
                    Err(_) => {},
                }
            },
        },

        Err(e) => utils::echoerr(
            lua,
            format!("Couldn't decode message from server: {e}"),
        )?,
    };

    Ok(())
}
