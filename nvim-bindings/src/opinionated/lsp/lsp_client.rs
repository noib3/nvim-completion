use std::sync::Arc;

use mlua::prelude::{LuaRegistryKey, LuaSerdeExt, LuaTable, LuaValue};
use tokio::sync::oneshot;

use super::{
    protocol::{CompletionResponse, ResponseError},
    LspError,
    LspMethod,
    LspResult,
};
use crate::opinionated::{BridgeRequest, LspHandler, LuaBridge};

/// Acts as an abstraction over a Neovim Lsp client (see `:h vim.lsp.client`).
#[derive(Debug)]
pub struct LspClient {
    bridge: Arc<LuaBridge>,

    // We have to store all functions through their registry key because
    // `mlua::Function`s are neither 'static nor Send.
    request_key: Arc<LuaRegistryKey>,
}

/// The function signature of an Lsp handler as defined by the Neovim API (see
/// `:h lsp-handler`). The tuple is `(err, result, ctx)`.
pub type LspHandlerSignature<'lua> =
    (Option<LuaTable<'lua>>, Option<LuaTable<'lua>>, LuaTable<'lua>);

impl LspClient {
    /// TODO: docs
    pub fn new(bridge: Arc<LuaBridge>, req_key: LuaRegistryKey) -> Self {
        Self { bridge, request_key: Arc::new(req_key) }
    }

    /// Binding to `vim.lsp.client.request`.
    pub async fn request(
        &self,
        method: LspMethod,
        bufnr: u16,
    ) -> LspResult<CompletionResponse> {
        let (tx, rx) = oneshot::channel::<LspResult<CompletionResponse>>();
        let mut tx = Some(tx);

        // This gets executed by Neovim when the response message arrives from
        // the server.
        let handler: LspHandler =
            Box::new(move |lua, (maybe_err, maybe_result, _ctx)| {
                let result = match maybe_result {
                    Some(table) => {
                        // TODO: why doesn't this work?
                        // Ok(lua.from_value::<CompletionResponse>(
                        //     LuaValue::Table(table),
                        // )?)

                        use super::protocol::{
                            CompletionItem,
                            CompletionList,
                        };

                        Ok(match table.get::<_, bool>("isIncomplete") {
                            Ok(_) => CompletionResponse::CompletionList(
                                lua.from_value::<CompletionList>(
                                    LuaValue::Table(table),
                                )?,
                            ),

                            Err(_) => CompletionResponse::CompletionItems(
                                lua.from_value::<Vec<CompletionItem>>(
                                    LuaValue::Table(table),
                                )?,
                            ),
                        })
                    },

                    None => Err(LspError::ResponseError(
                        lua.from_value::<ResponseError>(LuaValue::Table(
                            maybe_err.expect("no result so there's an error"),
                        ))?,
                    )),
                };

                let _ = tx
                    .take()
                    .expect("this only gets called once")
                    .send(result);

                Ok(())
            });

        let (responder, receiver) = oneshot::channel();

        let request = BridgeRequest::LspClientRequest {
            req_key: self.request_key.clone(),
            handler,
            method,
            bufnr,
            responder,
        };

        let _request_id = self.bridge.send(request, receiver).await?;

        rx.await?
    }
}
