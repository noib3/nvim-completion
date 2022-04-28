use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
    ToLua,
};

fn nvim(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")
}

/// Binding to `vim.inspect`.
pub fn inspect<'lua, V: ToLua<'lua>>(
    lua: &'lua Lua,
    v: V,
) -> LuaResult<String> {
    self::nvim(lua)?
        .get::<_, Table>("inspect")?
        .get::<_, LuaFunction>("inspect")?
        .call(v.to_lua(lua)?)
}

/// Binding to `_G.print`.
pub fn print<S: std::fmt::Display>(lua: &Lua, msg: S) -> LuaResult<()> {
    lua.globals().get::<_, LuaFunction>("print")?.call(msg.to_string())
}

/// Binding to `vim.schedule`.
pub fn schedule(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    self::nvim(lua)?.get::<_, LuaFunction>("schedule")?.call(callback)
}
