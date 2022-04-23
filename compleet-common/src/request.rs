use std::sync::Arc;

use bindings::{api, lsp};
use mlua::prelude::{
    Lua,
    LuaFunction,
    LuaRegistryKey,
    LuaResult,
    LuaTable,
    LuaValue,
};
use tokio::sync::oneshot;

use super::lsp::{LspClient, LspHandlerSignature, LspMethod};
use crate::bridge::LuaBridge;

pub type FuncMut = Box<
    dyn 'static
        + Send
        + for<'callback> FnMut(
            &'callback Lua,
            LspHandlerSignature<'callback>,
        ) -> LuaResult<()>,
>;

pub type Responder<T> = oneshot::Sender<T>;

// #[derive(Debug)]
pub enum BridgeRequest {
    ApiBufGetName {
        bufnr: u16,
        responder: Responder<String>,
    },

    ApiGetCurrentBuf {
        responder: Responder<u16>,
    },

    LspBufGetClients {
        bufnr: u16,
        responder: Responder<Vec<LspClient>>,
    },

    LspClientRequest {
        func_key: Arc<LuaRegistryKey>,
        method: LspMethod,
        handler: FuncMut,
        bufnr: u16,
        responder: Responder<u32>,
    },
}

impl BridgeRequest {
    /// Handles a request coming from ??
    pub fn handle(self, lua: &Lua) -> LuaResult<()> {
        use BridgeRequest::*;

        match self {
            ApiBufGetName { bufnr, responder } => {
                let filepath = api::buf_get_name(lua, bufnr)?;
                let _ = responder.send(filepath);
            },

            ApiGetCurrentBuf { responder } => {
                let bufnr = api::get_current_buf(lua)?;
                let _ = responder.send(bufnr);
            },

            LspBufGetClients { bufnr, responder } => {
                let client_tables = lsp::buf_get_clients(lua, bufnr)?
                    .sequence_values::<LuaTable>()
                    .filter_map(|table_res| table_res.ok())
                    .collect::<Vec<LuaTable>>();

                if client_tables.is_empty() {
                    let _ = responder.send(Vec::new());
                    return Ok(());
                }

                let bridge = Arc::new(LuaBridge::new(lua)?);

                let mut clients =
                    Vec::<LspClient>::with_capacity(client_tables.len());

                for table in client_tables {
                    let req = table.get::<_, LuaFunction>("request")?;

                    clients.push(LspClient::new(
                        bridge.clone(),
                        lua.create_registry_value(req)?,
                    ))
                }

                let _ = responder.send(clients);
            },

            LspClientRequest {
                func_key,
                method,
                handler,
                bufnr,
                responder,
            } => {
                let request = lua.registry_value::<LuaFunction>(&func_key)?;
                let (method_name, params) = method.expand(lua)?;
                let handler = lua.create_function_mut(handler)?;

                let _ = match request.call((
                    method_name,
                    params,
                    handler,
                    bufnr,
                ))? {
                    // Request failed, i.e. client has shutdown.
                    // TODO: return error message.
                    (LuaValue::Boolean(_), LuaValue::Nil) => responder.send(0),

                    // Request succeeded, a request id is returned.
                    (LuaValue::Boolean(_), LuaValue::Integer(req_id)) => {
                        responder.send(req_id as u32)
                    },

                    _ => unreachable!(),
                };
            },
        }

        Ok(())
    }
}
