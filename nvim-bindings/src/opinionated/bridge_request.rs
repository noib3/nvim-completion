use std::fmt;
use std::sync::Arc;

use mlua::prelude::{Lua, LuaFunction, LuaRegistryKey, LuaResult, LuaTable};
use tokio::sync::oneshot;

use super::{
    lsp::{
        protocol::CompletionParams,
        LspClient,
        LspError,
        LspHandlerSignature,
        LspResult,
    },
    LuaBridge,
};
use crate::{api, lsp};

// TODO: make the argument of the closure generic over `FromLuaMulti`?
pub type LspHandler = Box<
    dyn 'static
        + Send
        + for<'callback> FnMut(
            &'callback Lua,
            LspHandlerSignature<'callback>,
        ) -> LuaResult<()>,
>;

pub type Responder<T> = oneshot::Sender<T>;

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
        bridge: Arc<LuaBridge>,
        responder: Responder<Vec<LspClient>>,
    },

    LspClientRequestCompletions {
        req_key: Arc<LuaRegistryKey>,
        params: CompletionParams,
        handler: LspHandler,
        bufnr: u16,
        responder: Responder<LspResult<u32>>,
    },
}

impl fmt::Debug for BridgeRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("BridgeRequest")
    }
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

            LspBufGetClients { bufnr, bridge, responder } => {
                let client_tables = lsp::buf_get_clients(lua, bufnr)?
                    .sequence_values::<LuaTable>()
                    .filter_map(|table_res| table_res.ok())
                    .collect::<Vec<LuaTable>>();

                if client_tables.is_empty() {
                    let _ = responder.send(Vec::new());
                    return Ok(());
                }

                let mut clients =
                    Vec::<LspClient>::with_capacity(client_tables.len());

                for table in client_tables {
                    let req = table.get::<_, LuaFunction>("request")?;
                    let id = table.get("id")?;
                    let name = table.get("name")?;
                    let offset_encoding = table.get("offset_encoding")?;

                    clients.push(LspClient::new(
                        bridge.clone(),
                        lua.create_registry_value(req)?,
                        id,
                        name,
                        offset_encoding,
                    ))
                }

                let _ = responder.send(clients);
            },

            LspClientRequestCompletions {
                req_key,
                params,
                handler,
                bufnr,
                responder,
            } => {
                let request = lua.registry_value::<LuaFunction>(&req_key)?;
                let handler = lua.create_function_mut(handler)?;

                let _ = responder.send(
                    match request.call::<_, (bool, _)>((
                        "textDocument/completion".to_string(),
                        params,
                        handler,
                        bufnr,
                    ))? {
                        // Request failed, i.e. client has shutdown.
                        (_false, None) => Err(LspError::ClientShutdown),

                        // Request was sent, a request id is returned.
                        (_true, Some(id)) => Ok(id),
                    },
                );
            },
        }

        Ok(())
    }
}
