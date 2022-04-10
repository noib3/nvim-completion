use mlua::{
    prelude::{Lua, LuaFunction, LuaResult, LuaValue},
    Table,
};

fn nvim(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")
}

#[allow(dead_code)]
/// Binding to `vim.inspect`.
pub fn inspect(lua: &Lua, t: LuaValue) -> LuaResult<String> {
    self::nvim(lua)?
        .get::<_, Table>("inspect")?
        .get::<_, LuaFunction>("inspect")?
        .call(t)
}

#[allow(dead_code)]
/// Binding to `_G.print`.
pub fn print<S: std::fmt::Display>(lua: &Lua, msg: S) -> LuaResult<()> {
    lua.globals()
        .get::<_, LuaFunction>("print")?
        .call(msg.to_string())
}

/// Binding to `vim.schedule`.
pub fn schedule(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    self::nvim(lua)?
        .get::<_, LuaFunction>("schedule")?
        .call(callback)
}
