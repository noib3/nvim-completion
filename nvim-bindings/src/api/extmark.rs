use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

/// Binding to `vim.api.nvim_buf_add_highlight`.
pub fn buf_add_highlight(
    lua: &Lua,
    bufnr: u16,
    ns_id: i32,
    hl_group: String,
    line: u32,
    col_start: u32,
    col_end: i32,
) -> LuaResult<i32> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_buf_add_highlight")?
        .call::<_, i32>((bufnr, ns_id, hl_group, line, col_start, col_end))
}

/// Binding to `vim.api.nvim_buf_clear_namespace`.
pub fn buf_clear_namespace(
    lua: &Lua,
    bufnr: u16,
    ns_id: i32,
    line_start: u32,
    line_end: i32,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_buf_clear_namespace")?
        .call::<_, ()>((bufnr, ns_id, line_start, line_end))
}

/// Binding to `vim.api.nvim_buf_set_extmark`.
pub fn buf_set_extmark(
    lua: &Lua,
    bufnr: u16,
    ns_id: u16,
    row: u16,
    col: u16,
    opts: Table,
) -> LuaResult<u16> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_buf_set_extmark")?
        .call((bufnr, ns_id, row, col, opts))
}

/// Binding to `vim.api.nvim_create_namespace`.
pub fn create_namespace(lua: &Lua, name: &'static str) -> LuaResult<u16> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_create_namespace")?.call(name)
}
