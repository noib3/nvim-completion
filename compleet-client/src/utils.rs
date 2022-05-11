use bindings::{api, r#fn};
use mlua::prelude::{Lua, LuaResult};

pub fn get_screen_cursor(lua: &Lua) -> LuaResult<(u16, u16)> {
    let (row, col) = api::win_get_cursor(lua, 0)?;
    let pos = r#fn::screenpos(lua, row, col + 1)?;
    Ok((pos.get::<_, u16>("row")?, pos.get::<_, u16>("col")? - 1))
}
