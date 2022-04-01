use mlua::prelude::{Lua, LuaFunction, LuaResult};

use crate::channel::Channel;

pub struct State {
    pub channel: Channel,
}

impl State {
    pub fn new(
        lua: &Lua,
        on_exit: LuaFunction,
        on_stderr: LuaFunction,
    ) -> LuaResult<State> {
        let channel = Channel::new(lua, on_exit, on_stderr)?;

        Ok(State { channel })
    }
}
