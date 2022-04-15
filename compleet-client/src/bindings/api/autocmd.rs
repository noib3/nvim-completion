use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

/// Binding to `vim.api.nvim_clear_autocmds`.
pub fn clear_autocmds(lua: &Lua, opts: Table) -> LuaResult<()> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_clear_autocmds")?
        .call(opts)
}

/// Binding to `vim.api.nvim_create_augroup`.
pub fn create_augroup(lua: &Lua, name: &str, opts: Table) -> LuaResult<u32> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_create_augroup")?
        .call((name, opts))
}

/// Binding to `vim.api.nvim_create_autocmd`.
pub fn create_autocmd<S: AsRef<str>, E: IntoIterator<Item = S>>(
    lua: &Lua,
    events: E,
    opts: Table,
) -> LuaResult<u32> {
    let events =
        events.into_iter().map(|e| e.as_ref().into()).collect::<Vec<String>>();

    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_create_autocmd")?
        .call((events, opts))
}

/// Binding to `vim.api.nvim_del_augroup_by_id`.
pub fn del_augroup_by_id(lua: &Lua, id: u32) -> LuaResult<()> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_del_augroup_by_id")?
        .call(id)
}

#[allow(dead_code)]
/// Binding to `vim.api.nvim_del_augroup_by_name`.
pub fn del_augroup_by_name(lua: &Lua, name: &str) -> LuaResult<()> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_del_augroup_by_name")?
        .call(name)
}

#[allow(dead_code)]
/// Binding to `vim.api.nvim_del_autocmd`.
pub fn del_autocmd(lua: &Lua, id: u32) -> LuaResult<()> {
    super::api(lua)?.get::<&str, LuaFunction>("nvim_del_autocmd")?.call(id)
}

/// Binding to `vim.api.nvim_exec_autocmds`.
pub fn exec_autocmds<S: AsRef<str>, E: IntoIterator<Item = S>>(
    lua: &Lua,
    events: E,
    opts: Table,
) -> LuaResult<()> {
    let events =
        events.into_iter().map(|e| e.as_ref().into()).collect::<Vec<String>>();

    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_exec_autocmds")?
        .call((events, opts))
}

#[allow(dead_code)]
/// Binding to `vim.api.nvim_get_autocmds`.
pub fn get_autocmds<'lua>(
    lua: &'lua Lua,
    opts: Table<'lua>,
) -> LuaResult<Table<'lua>> {
    super::api(lua)?.get::<&str, LuaFunction>("nvim_get_autocmds")?.call(opts)
}
