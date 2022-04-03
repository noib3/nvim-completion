use compleet::rpc::message::*;
use mlua::prelude::{Lua, LuaResult};

use crate::state::State;
use crate::utils;

/// Called when ...
pub fn on_stderr(
    lua: &Lua,
    _state: &mut State,
    bytes: Vec<u8>,
) -> LuaResult<()> {
    let _ciao = bytes.clone();

    match RpcMessage::try_from(bytes) {
        Ok(message) => match message {
            RpcMessage::Request(_req) => todo!(),
            RpcMessage::Response(_rsp) => todo!(),
            RpcMessage::Notification(_ntf) => todo!(),
        },

        Err(e) => utils::echoerr(
            lua,
            vec![(
                // &format!("Couldn't decode message from server: {:?}",
                // ciao),
                &format!("Couldn't decode message from server: {e}"),
                None,
            )],
        )?,
    };

    Ok(())
}
