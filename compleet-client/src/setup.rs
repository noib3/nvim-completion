use mlua::prelude::{Lua, LuaResult, LuaValue};

pub fn setup(lua: &Lua, _preferences: LuaValue) -> LuaResult<()> {
    let print = lua.globals().get::<_, mlua::Function>("print")?;

    print.call::<_, ()>("Setup complete!")
}
