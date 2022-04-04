use std::cell::RefCell;
use std::rc::Rc;

use mlua::{
    prelude::{LuaError, LuaResult, LuaValue},
    Lua,
};

use super::message::{Notification, Request};
use crate::bindings::{api, nvim, r#fn};
use crate::constants::*;
use crate::state::State;

#[derive(Debug)]
pub struct Channel(u32);

impl Channel {
    /// Spawns a new RPC channel via `vim.fn.jobstart`.
    pub fn new(lua: &Lua, state: &Rc<RefCell<State>>) -> LuaResult<Channel> {
        let cloned = state.clone();
        let on_exit =
            lua.create_function(move |lua, (_id, code): (u32, _)| {
                super::on_exit(lua, &mut cloned.borrow_mut(), code)
            })?;

        let cloned = state.clone();
        let on_stderr = lua.create_function(
            move |lua, (_id, data): (u32, Vec<mlua::String>)| {
                // Convert the received data from a vector of Lua strings to a
                // vector of raw bytes.
                let bytes = data
                    .into_iter()
                    .map(|s| s.as_bytes().to_vec())
                    .intersperse(vec![b'\n'])
                    .flatten()
                    .collect::<Vec<u8>>();

                super::on_stderr(lua, &mut cloned.borrow_mut(), bytes)
            },
        )?;

        let opts = lua.create_table_from([
            ("on_exit", LuaValue::Function(on_exit)),
            ("on_stderr", LuaValue::Function(on_stderr)),
            ("rpc", LuaValue::Boolean(true)),
        ])?;

        let path = self::compleet_server_path(lua)?;

        let id = match r#fn::jobstart(lua, &[path], opts)? {
            -1 => {
                // TODO: custom error
                return Err(LuaError::RuntimeError(format!(
                    "The `{COMPLEET_SERVER_BINARY_NAME}` binary is not \
                     executable!"
                )));
            },

            id => id as u32,
        };

        Ok(Channel(id))
    }

    /// Sends a notification to the server.
    pub fn notify(&self, lua: &Lua, ntf: Notification) -> LuaResult<()> {
        let (event, args) = ntf.into();
        nvim::rpcnotify(lua, self.0, event, args)
    }

    /// Sends a request to the server and waits for the response.
    pub fn _request(&self, lua: &Lua, req: Request) -> LuaResult<()> {
        let (method, args) = req.into();
        nvim::rpcrequest(lua, self.0, method, args)
    }
}

/// Returns the full path of the compleet server binary.
fn compleet_server_path(lua: &Lua) -> LuaResult<String> {
    match api::get_runtime_file(
        lua,
        &format!("lua/{COMPLEET_SERVER_BINARY_NAME}"),
        false,
    )? {
        // TODO: custom error
        vec if vec.is_empty() => Err(LuaError::RuntimeError(format!(
            "Couldn't find the `{COMPLEET_SERVER_BINARY_NAME}` binary :("
        ))),

        vec => Ok(vec.into_iter().nth(0).expect("Already checked empty")),
    }
}
