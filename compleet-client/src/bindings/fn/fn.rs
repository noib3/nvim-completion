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

/// Binding to `vim.fn.screenpos
pub fn screenpos(lua: &Lua, row: u16, col: u16) -> LuaResult<(u16, u16)> {
    let pos = self::r#fn(lua)?
        .get::<_, LuaFunction>("screenpos")?
        .call::<_, Table>((0, row, col + 1))?;

    Ok((pos.get::<_, u16>("row")?, pos.get::<_, u16>("col")? - 1))
}

/// Binding to `vim.fn.winlayout
pub fn winlayout(lua: &Lua) -> LuaResult<Table> {
    self::r#fn(lua)?.get::<_, LuaFunction>("winlayout")?.call::<_, Table>(())
}
