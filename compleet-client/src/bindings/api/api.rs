use mlua::{prelude::LuaResult, Lua, Table};

pub fn api(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")?.get::<_, Table>("api")
}
