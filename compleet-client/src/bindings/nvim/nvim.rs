use mlua::{
    prelude::{Lua, LuaFunction, LuaResult, LuaValue},
    MultiValue,
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

/// Binding to `vim.rpcnotify`.
pub fn rpcnotify(
    lua: &Lua,
    channel: u32,
    event: String,
    args: Vec<LuaValue>,
) -> LuaResult<()> {
    self::nvim(lua)?.get::<_, LuaFunction>("rpcnotify")?.call((
        channel,
        event,
        MultiValue::from_vec(args),
    ))
}

/// Binding to `vim.rpcrequest`.
pub fn rpcrequest(
    lua: &Lua,
    channel: u32,
    method: String,
    args: Vec<LuaValue>,
) -> LuaResult<()> {
    self::nvim(lua)?.get::<_, LuaFunction>("rpcrequest")?.call((
        channel,
        method,
        MultiValue::from_vec(args),
    ))
}

/// Binding to `vim.schedule`.
pub fn schedule(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    self::nvim(lua)?
        .get::<_, LuaFunction>("schedule")?
        .call(callback)
}
