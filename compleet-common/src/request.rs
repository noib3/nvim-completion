use bindings::{api, lsp};
use mlua::prelude::{Lua, LuaResult};

use tokio::sync::oneshot;

pub type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum Request {
    /// Requests `vim.api.nvim_get_current_buf`.
    ApiGetCurrentBuf(Responder<u16>),

    /// Requests `vim.lsp.buf_get_clients`.
    LspBufGetClients(u16, Responder<u32>),
}

impl Request {
    pub fn handle(self, lua: &Lua) -> LuaResult<()> {
        use Request::*;

        match self {
            ApiGetCurrentBuf(resp) => {
                let bufnr = api::get_current_buf(lua)?;
                let _ = resp.send(bufnr);
            },

            LspBufGetClients(bufnr, resp) => {
                let len = lsp::buf_get_clients(lua, bufnr)?.len()? as u32;
                let _ = resp.send(len);
            },
        }

        Ok(())
    }
}
