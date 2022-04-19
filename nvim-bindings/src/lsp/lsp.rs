use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

fn lsp(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")?.get::<_, Table>("lsp")
}

/// Binding to `vim.lsp.buf_get_clients`.
pub fn buf_get_clients(lua: &Lua, bufnr: u16) -> LuaResult<Table> {
    self::lsp(lua)?.get::<_, LuaFunction>("buf_get_clients")?.call(bufnr)
}
