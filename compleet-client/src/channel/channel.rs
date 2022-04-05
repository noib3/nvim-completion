use std::cell::RefCell;
use std::rc::Rc;

use compleet::api::incoming::{Notification, Request};
use compleet::rpc::RpcMessage;
use mlua::prelude::{
    FromLua,
    Lua,
    LuaError,
    LuaResult,
    LuaSerdeExt,
    LuaString,
    LuaValue,
};

use crate::bindings::{api, nvim, r#fn};
use crate::constants::*;
use crate::state::State;

#[derive(Debug, Default)]
pub struct Channel(u32);

impl Channel {
    /// Opens a new RPC channel via `vim.fn.jobstart`.
    pub fn new(lua: &Lua, state: &Rc<RefCell<State>>) -> LuaResult<Channel> {
        let cloned = state.clone();
        let on_exit =
            lua.create_function(move |lua, (_id, code): (u32, _)| {
                super::on_exit(lua, &mut cloned.borrow_mut(), code)
            })?;

        let cloned = state.clone();
        let on_stderr = lua.create_function(
            move |lua, (_id, data): (u32, Vec<LuaString>)| {
                // Convert the received data from Lua strings to raw bytes.
                let bytes = data
                    .into_iter()
                    .map(|s| s.as_bytes().to_vec())
                    // Re-add a newline byte between every string byte chunk.
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
                    "The `{SERVER_BINARY_NAME}` binary is not executable!"
                )));
            },

            id => id as u32,
        };

        Ok(Channel(id))
    }

    /// Sends a notification to the server.
    pub fn notify(&self, lua: &Lua, ntf: Notification) -> LuaResult<()> {
        // TODO: this is ugly
        let (method, params) = match RpcMessage::from(ntf) {
            RpcMessage::Notification { method, params } => {
                let params = params
                    .into_iter()
                    .flat_map(|v| lua.to_value(&v))
                    .collect::<Vec<LuaValue>>();

                (method, params)
            },
            _ => unreachable!(),
        };

        nvim::rpcnotify(lua, self.0, method, params)
    }

    /// Sends a request to the server and blocks until the response is
    /// received.
    pub fn request<'lua, T: FromLua<'lua>>(
        &self,
        _lua: &'lua Lua,
        _req: Request,
    ) -> LuaResult<T> {
        // let (method, params) = match RpcMessage::from(req) {
        //     RpcMessage::Request {
        //         msgid: _,
        //         method,
        //         params,
        //     } => {
        //         let params = params
        //             .into_iter()
        //             .flat_map(|v| lua.to_value(&v))
        //             .collect::<Vec<LuaValue>>();

        //         (method, params)
        //     },
        //     _ => unreachable!(),
        // };

        // nvim::rpcrequest(lua, self.0, method, params)

        todo!()
    }
}

/// Returns the full path of the compleet server binary.
fn compleet_server_path(lua: &Lua) -> LuaResult<String> {
    match api::get_runtime_file(
        lua,
        &format!("lua/{SERVER_BINARY_NAME}"),
        false,
    )? {
        // TODO: custom error
        vec if vec.is_empty() => Err(LuaError::RuntimeError(format!(
            "Couldn't find the `{SERVER_BINARY_NAME}` binary :("
        ))),

        vec => Ok(vec.into_iter().nth(0).expect("Already checked empty")),
    }
}
