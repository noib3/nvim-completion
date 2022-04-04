use compleet::rpc::message::*;
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
            RpcMessage::Request(_req) => {},

            RpcMessage::Response(_rsp) => {},

            RpcMessage::Notification(ntf) => {
                super::handle_notify(lua, state, ntf)?
            },
        },

        Err(e) => utils::echoerr(
            lua,
            vec![(&format!("Couldn't decode message from server: {e}"), None)],
        )?,
    };

    Ok(())
}
