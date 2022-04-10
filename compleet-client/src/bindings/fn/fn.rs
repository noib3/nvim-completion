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

/// Binding to `vim.fn.screenrow`.
pub fn screenrow(lua: &Lua) -> LuaResult<u16> {
    self::r#fn(lua)?
        .get::<_, LuaFunction>("screenrow")?
        .call(())
}
