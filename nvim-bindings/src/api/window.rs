use mlua::{
    prelude::{LuaFunction, LuaResult},
    FromLua,
    Lua,
    Table,
    ToLua,
};

/// Binding to `vim.api.nvim_win_close`.
pub fn win_close(lua: &Lua, winid: u16, force: bool) -> LuaResult<()> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_close")?
        .call((winid, force))
}

/// Binding to `vim.api.nvim_win_get_cursor`
pub fn win_get_cursor(lua: &Lua, winid: u16) -> LuaResult<(u16, u16)> {
    let position = super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_get_cursor")?
        .call::<_, Table>(winid)?;

    Ok((position.get(1)?, position.get(2)?))
}

/// Binding to `vim.api.nvim_win_get_option`
pub fn win_get_option<'lua, V: FromLua<'lua>>(
    lua: &'lua Lua,
    winid: u16,
    name: &str,
) -> LuaResult<V> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_get_option")?
        .call((winid, name))
}

/// Binding to `vim.api.nvim_win_get_position`
pub fn win_get_position(lua: &Lua, winid: u16) -> LuaResult<(u16, u16)> {
    let position = super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_get_position")?
        .call::<_, Table>(winid)?;

    Ok((position.get(1)?, position.get(2)?))
}

/// Binding to `vim.api.nvim_win_get_width`
pub fn win_get_width(lua: &Lua, winid: u16) -> LuaResult<u16> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_win_get_width")?.call(winid)
}

/// Binding to `vim.api.nvim_win_get_width`
pub fn win_get_height(lua: &Lua, winid: u16) -> LuaResult<u16> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_win_get_height")?.call(winid)
}

/// Binding to `vim.api.nvim_win_hide`.
pub fn win_hide(lua: &Lua, winid: u16) -> LuaResult<()> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_win_hide")?.call(winid)
}

/// Binding to `vim.api.nvim_win_set_cursor`
pub fn win_set_cursor(
    lua: &Lua,
    winid: u16,
    row: u16,
    col: u16,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_set_cursor")?
        .call((winid, [row, col]))
}

/// Binding to `vim.api.nvim_win_set_option`
pub fn win_set_option<'lua, V: ToLua<'lua>>(
    lua: &'lua Lua,
    winid: u16,
    name: &str,
    value: V,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_set_option")?
        .call((winid, name, value))
}
