use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;
use crate::constants;

pub fn echoerr(lua: &Lua, msg: Vec<(&str, Option<&str>)>) -> LuaResult<()> {
    let chunks = [
        ("[nvim-compleet]", Some(constants::ERROR_MSG_HLGROUP_NAME)),
        (" ", None),
    ]
    .into_iter()
    .chain(msg.into_iter())
    .collect::<Vec<(&str, Option<&str>)>>();

    api::echo(lua, chunks, true)
}
