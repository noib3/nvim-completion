use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

fn r#fn(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")?.get::<_, Table>("fn")
}

/// Binding to `vim.fn.has`.
pub fn has(lua: &Lua, feature: &str) -> LuaResult<bool> {
    let bit = self::r#fn(lua)?
        .get::<_, LuaFunction>("has")?
        .call::<_, u8>(feature)?;

    Ok(bit == 1)
}

/// Binding to `vim.fn.screenrow`
pub fn screenrow(lua: &Lua) -> LuaResult<u16> {
    Ok(self::r#fn(lua)?
        .get::<_, LuaFunction>("screenrow")?
        .call::<_, u16>(())?)
}

/// Binding to `vim.fn.screenpos`
pub fn screencol(lua: &Lua) -> LuaResult<u16> {
    Ok(self::r#fn(lua)?
        .get::<_, LuaFunction>("screencol")?
        .call::<_, u16>(())?)
}

/// Binding to `vim.fn.winlayout`
pub fn winlayout(lua: &Lua) -> LuaResult<Table> {
    self::r#fn(lua)?.get::<_, LuaFunction>("winlayout")?.call::<_, Table>(())
}
