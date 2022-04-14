use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    AnyUserData,
    Table,
};

fn r#loop(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")?.get::<_, Table>("loop")
}

/// Binding to `vim.loop.new_signal()`.
pub fn new_signal(lua: &Lua) -> LuaResult<AnyUserData> {
    self::r#loop(lua)?.get::<_, LuaFunction>("new_signal")?.call(())
}

/// Binding to `vim.loop.signal_start()`.
pub fn signal_start(
    lua: &Lua,
    signal: AnyUserData,
    signum: &'static str,
    callback: LuaFunction,
) -> LuaResult<()> {
    self::r#loop(lua)?
        .get::<_, LuaFunction>("new_signal")?
        .call((signal, signum, callback))
}
