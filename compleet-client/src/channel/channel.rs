use mlua::{
    prelude::{LuaError, LuaResult, LuaValue},
    Lua,
};

use crate::bindings::{api, r#fn};

#[derive(Debug)]
pub struct Channel {
    _id: u32,
}

impl Channel {
    /// Spawns a new RPC channel via `vim.fn.jobstart`.
    pub fn new(lua: &Lua) -> LuaResult<Channel> {
        let path = get_compleet_server_path(lua)?;

        let on_exit = lua.create_function(|lua, (id, code)| {
            super::on_exit(lua, id, code)
        })?;

        let on_stderr = lua.create_function(|lua, (id, data)| {
            super::on_stderr(lua, id, data)
        })?;

        let opts = lua.create_table_from([
            ("on_exit", LuaValue::Function(on_exit)),
            ("on_stderr", LuaValue::Function(on_stderr)),
            ("rpc", LuaValue::Boolean(true)),
            // ("stderr_buffered", LuaValue::Boolean(true)),
        ])?;

        let id = match r#fn::jobstart(lua, &[path], opts)? {
            -1 => {
                return Err(LuaError::RuntimeError(
                    "The `compleet-server` binary is not executable!".into(),
                ))
            },

            id => id as u32,
        };

        Ok(Channel { _id: id })
    }
}

/// Returns the full path of the `compleet-server` binary.
fn get_compleet_server_path(lua: &Lua) -> LuaResult<String> {
    match api::get_runtime_file(lua, "lua/compleet-server", false)? {
        v if v.is_empty() => Err(LuaError::RuntimeError(
            "Couldn't find the `compleet-server` binary :(".into(),
        )),

        v => Ok(v.into_iter().nth(0).expect("Already checked empty variant")),
    }
}
