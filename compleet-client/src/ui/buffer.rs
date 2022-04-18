use std::fmt::{self, Display};

use mlua::prelude::{FromLua, Lua, LuaFunction, LuaResult, LuaValue};

use crate::bindings::api;

#[derive(Debug, PartialEq)]
pub struct Buffer {
    pub bufnr: u16,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.bufnr.to_string())
    }
}

impl Buffer {
    /// Returns the current buffer.
    pub fn get_current(lua: &Lua) -> LuaResult<Self> {
        Ok(Self { bufnr: api::get_current_buf(lua)? })
    }

    /// Get a buffer-local option.
    pub fn get_option<'lua, V: FromLua<'lua>>(
        &self,
        lua: &'lua Lua,
        name: &str,
    ) -> LuaResult<V> {
        api::buf_get_option::<V>(lua, self.bufnr, name)
    }

    /// Calls `vim.api.nvim_buf_attach` on the buffer with the `on_bytes`
    /// callback.
    pub fn attach(&self, lua: &Lua, on_bytes: LuaFunction) -> LuaResult<bool> {
        let opts = lua.create_table_from([
            // Fuck off rustfmt.
            ("on_bytes", LuaValue::Function(on_bytes)),
        ])?;

        api::buf_attach(lua, self.bufnr, false, opts)
    }
}
