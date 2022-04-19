use std::fmt::{self, Display};

use bindings::api;
use mlua::prelude::{FromLua, Lua, LuaFunction, LuaResult, LuaValue};

#[derive(Debug, PartialEq)]
pub struct Buffer {
    pub number: u16,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.number.to_string())
    }
}

impl Buffer {
    /// Returns the current buffer.
    pub fn get_current(lua: &Lua) -> LuaResult<Self> {
        Ok(Self { number: api::get_current_buf(lua)? })
    }

    /// Get a buffer-local option.
    pub fn get_option<'lua, V: FromLua<'lua>>(
        &self,
        lua: &'lua Lua,
        name: &str,
    ) -> LuaResult<V> {
        api::buf_get_option::<V>(lua, self.number, name)
    }

    /// Calls `vim.api.nvim_buf_attach` on the buffer with the `on_bytes`
    /// callback.
    pub fn attach(&self, lua: &Lua, on_bytes: LuaFunction) -> LuaResult<bool> {
        let opts = lua.create_table_from([
            // Fuck off rustfmt.
            ("on_bytes", LuaValue::Function(on_bytes)),
        ])?;

        api::buf_attach(lua, self.number, false, opts)
    }
}
