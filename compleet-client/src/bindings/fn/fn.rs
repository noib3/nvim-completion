use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

fn r#fn(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")?.get::<_, Table>("fn")
}

pub fn has(lua: &Lua, feature: &str) -> LuaResult<bool> {
    let flag = self::r#fn(lua)?
        .get::<_, LuaFunction>("has")?
        .call::<_, u8>(feature)?;

    Ok(flag == 1)
}

pub fn jobstart(lua: &Lua, cmd: &[String], opts: Table) -> LuaResult<i64> {
    self::r#fn(lua)?
        .get::<_, LuaFunction>("jobstart")?
        .call((cmd, opts))
}
