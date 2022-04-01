use mlua::prelude::{Lua, LuaResult};

use crate::State;

pub fn update(_lua: &Lua, _state: &mut State) -> LuaResult<()> { Ok(()) }
