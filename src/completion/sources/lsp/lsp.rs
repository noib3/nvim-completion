use async_trait::async_trait;
use mlua::prelude::LuaResult;
use neovim::Api;
use serde::Deserialize;

use crate::completion::{
    CompletionItem,
    CompletionSource,
    Completions,
    Cursor,
};

#[derive(Debug, Deserialize)]
pub struct Lsp {
    pub enable: bool,
}

impl Default for Lsp {
    fn default() -> Self { Lsp { enable: false } }
}

/*
TODOs:

1. implement attach: check if buffer has any clients associated with it, if it
   does somehow save their id's (pid, rpc id, idk) somewhere. In complete go
   over those `ids` to get the currently attached sources. For every source
   send a completion request.

Communication with server happens over the lsp's stdio. The lsp process is
spawned in `neovim/runtime/lua/vim/lsp/rpc.lua:327` via `uv.spawn`. That
returns a `handle` and a `pid`

2. remove source on `LspStop` or if the server quits

3. add the source on `LspStart`
*/

#[async_trait]
impl CompletionSource for Lsp {
    fn attach(&self, _: &Api, _bufnr: u32) -> LuaResult<bool> {
        // let clients = nvim.lsp.buf_get_clients(bufnr)?;
        Ok(true)
    }

    async fn complete(&self, _cursor: &Cursor) -> Completions {
        // Simulate a slow source, this shouldn't block.
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        // std::thread::sleep(std::time::Duration::from_secs(5));

        let item = CompletionItem {
            details: None,
            format: " Lsp".into(),
            hl_ranges: vec![],
            matched_bytes: 1,
            source: "Lsp",
            text: "Lsp".into(),
        };

        vec![item]
    }
}
