use mlua::prelude::{Lua, LuaResult};

// use crate::utils;

/// Called when .
pub fn on_stderr(
    lua: &Lua,
    _channel_id: u32,
    data: Vec<String>,
) -> LuaResult<()> {
    let print = lua.globals().get::<_, mlua::Function>("print")?;
    print.call::<_, ()>(format!("got something!: {:?}", data))
}
