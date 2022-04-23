use std::sync::Arc;

use mlua::prelude::{Lua, LuaResult};
use tokio::sync::oneshot;

use super::{lsp, BridgeRequest, LuaBridge};

#[derive(Debug)]
pub struct Neovim {
    bridge: Arc<LuaBridge>,
}

impl Neovim {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        Ok(Self { bridge: Arc::new(LuaBridge::new(lua)?) })
    }

    /// Binding to `vim.api.nvim_buf_get_name`.
    pub async fn api_buf_get_name(&self, bufnr: u16) -> String {
        let (responder, receiver) = oneshot::channel();
        let request = BridgeRequest::ApiBufGetName { bufnr, responder };
        self.bridge.send(request, receiver).await
    }

    /// Binding to `vim.api.nvim_get_current_buf`.
    pub async fn api_get_current_buf(&self) -> u16 {
        let (responder, receiver) = oneshot::channel();
        let request = BridgeRequest::ApiGetCurrentBuf { responder };
        self.bridge.send(request, receiver).await
    }

    /// Binding to `vim.lsp.buf_get_clients`.
    pub async fn lsp_buf_get_clients(
        &self,
        bufnr: u16,
    ) -> Vec<lsp::LspClient> {
        let (responder, receiver) = oneshot::channel();
        let request = BridgeRequest::LspBufGetClients { bufnr, responder };
        self.bridge.send(request, receiver).await
    }
}
