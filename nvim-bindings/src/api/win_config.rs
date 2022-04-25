use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

/// Binding to `vim.api.nvim_open_win`.
pub fn open_win(
    lua: &Lua,
    bufnr: u16,
    enter: bool,
    config: Table,
) -> LuaResult<u16> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_open_win")?
        .call((bufnr, enter, config))
}

#[allow(dead_code)]
/// Binding to `vim.api.nvim_win_get_config`.
pub fn win_get_config(lua: &Lua, winid: u16) -> LuaResult<Table> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_win_get_config")?.call(winid)
}

/// Binding to `vim.api.nvim_win_set_config`.
pub fn win_set_config(lua: &Lua, winid: u16, config: Table) -> LuaResult<()> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_win_set_config")?
        .call((winid, config))
}
