use std::sync::Arc;

use mlua::prelude::{LuaRegistryKey, LuaTable};
use tokio::sync::oneshot;

use super::{LspError, LspMethod, LspResult};
use crate::opinionated::{BridgeRequest, FuncMut, LuaBridge};

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
    ) -> LspResult<u32> {
        let (tx, rx) = oneshot::channel::<LspResult<u32>>();
        let mut tx = Some(tx);

        let callback: FuncMut = Box::new(
            move |_lua, (maybe_err, maybe_result, _ctx): LspHandlerSignature| {
                let tx = tx.take().expect("this only gets called once");

                let _ = match maybe_result {
                    Some(result) => {
                        let num = result.get::<_, LuaTable>("items")?.len()?;
                        tx.send(Ok(num as u32))
                    },

                    None => {
                        let infos = maybe_err
                            .expect("no result so there's an error")
                            .get::<_, String>("message")?;

                        tx.send(Err(LspError::Any(infos)))
                    },
                };

                Ok(())
            },
        );

        let (responder, receiver) = oneshot::channel();

        let request = BridgeRequest::LspClientRequest {
            func_key: self.request_key.clone(),
            method,
            handler: callback,
            bufnr,
            responder,
        };

        let _req_id = self.bridge.send(request, receiver).await;

        match rx.await {
            Err(_) => Err(LspError::Any("Receiver dropped".into())),

            Ok(Err(_)) => Err(LspError::Any("Something".into())),

            Ok(Ok(bool)) => Ok(bool),
        }
    }
}
