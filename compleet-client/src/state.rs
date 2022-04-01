use mlua::prelude::{Lua, LuaResult};

use crate::channel::Channel;

pub struct State {
    pub channel: Option<Channel>,
}

impl State {
    pub fn new(_lua: &Lua) -> LuaResult<State> { Ok(State { channel: None }) }
}
