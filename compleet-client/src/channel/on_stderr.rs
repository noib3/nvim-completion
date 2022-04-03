use mlua::prelude::{Lua, LuaResult};

use crate::state::State;

/// Called when .
pub fn on_stderr(
    lua: &Lua,
    _state: &mut State,
    data: Vec<u8>,
) -> LuaResult<()> {
    let print = lua.globals().get::<_, mlua::Function>("print")?;
    print.call::<_, ()>(format!("got something!: {:?}", data))
}
