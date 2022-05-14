use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

fn lsp(lua: &Lua) -> LuaResult<Table> {
    lua.globals().get::<_, Table>("vim")?.get::<_, Table>("lsp")
}

/// Binding to `vim.lsp.buf_get_clients`.
pub fn buf_get_clients(lua: &Lua, bufnr: u32) -> LuaResult<Table> {
    self::lsp(lua)?.get::<_, LuaFunction>("buf_get_clients")?.call(bufnr)
}

/// Binding to `vim.lsp.protocol.make_client_capabilities`.
pub fn make_client_capabilities(lua: &Lua) -> LuaResult<Table> {
    self::lsp(lua)?
        .get::<_, Table>("protocol")?
        .get::<_, LuaFunction>("make_client_capabilities")?
        .call(())
}
