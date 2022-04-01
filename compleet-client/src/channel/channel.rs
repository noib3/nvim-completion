use mlua::prelude::{Lua, LuaError, LuaFunction, LuaResult, LuaValue};

pub struct Channel {
    id: u32,
}

impl Channel {
    /// Spawns a new RPC channel via `vim.fn.jobstart`.
    pub fn new(
        lua: &Lua,
        on_exit: LuaFunction,
        on_stderr: LuaFunction,
    ) -> LuaResult<Channel> {
        let jobstart = lua
            .globals()
            .get::<_, mlua::Table>("vim")?
            .get::<_, mlua::Table>("fn")?
            .get::<_, LuaFunction>("jobstart")?;

        let path = "/home/noib3/Dropbox/projects/nvim-complet/target/debug/\
                    compleet-server";

        let opts = lua.create_table_from([
            ("on_exit", LuaValue::Function(on_exit)),
            ("on_stderr", LuaValue::Function(on_stderr)),
            ("rpc", LuaValue::Boolean(true)),
            ("stderr_buffered", LuaValue::Boolean(true)),
        ])?;

        let id = match jobstart.call::<_, i64>((path, opts))? {
            -1 => {
                return Err(LuaError::RuntimeError(format!(
                    "The `compleet-server` binary at '{path}' either doesn't \
                     exist or it's not executable!"
                )))
            },
            id => id as u32,
        };

        Ok(Channel { id })
    }
}
