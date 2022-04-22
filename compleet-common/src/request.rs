use std::sync::Arc;

use bindings::{api, lsp};
use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};
use tokio::sync::oneshot;

use super::lsp::LspClient;
use crate::bridge::LuaBridge;

pub type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum Request {
    /// Binding to `vim.api.nvim_get_current_buf`.
    ApiGetCurrentBuf(Responder<u16>),

    /// Binding to `vim.lsp.buf_get_clients`.
    LspBufGetClients(u16, Responder<Vec<LspClient>>),
}

impl Request {
    /// Handles a request coming from ??
    pub fn handle(self, lua: &Lua) -> LuaResult<()> {
        use Request::*;

        match self {
            ApiGetCurrentBuf(resp) => {
                let bufnr = api::get_current_buf(lua)?;
                let _ = resp.send(bufnr);
            },

            LspBufGetClients(bufnr, resp) => {
                let client_tables = lsp::buf_get_clients(lua, bufnr)?
                    .sequence_values::<Table>()
                    .filter_map(|table_res| table_res.ok())
                    .collect::<Vec<Table>>();

                if client_tables.is_empty() {
                    let _ = resp.send(Vec::new());
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

                let _ = resp.send(clients);
            },
        }

        Ok(())
    }
}
