use mlua::prelude::{Lua, LuaResult};

use crate::utils;

/// Called when the RPC channel gets closed.
pub fn on_exit(lua: &Lua, _channel_id: u32, exit_code: u32) -> LuaResult<()> {
    match exit_code {
        // 0 | 143 => Ok(()),
        143 => Ok(()),
        num => utils::echoerr(
            lua,
            vec![
                ("The server just quit with exit code ", None),
                (&num.to_string(), Some("Visual")),
            ],
        ),
    }
}
