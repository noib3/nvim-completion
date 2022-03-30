use mlua::prelude::LuaResult;
use neovim::Api;
use serde::Deserialize;

use crate::completion::{CompletionSource, Completions, Cursor};

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

impl CompletionSource for Lsp {
    fn attach(&self, _: &Api, _bufnr: u32) -> LuaResult<bool> {
        // let clients = nvim.lsp.buf_get_clients(bufnr)?;
        Ok(true)
    }

    fn complete(&self, _: &Api, _cursor: &Cursor) -> LuaResult<Completions> {
        Ok(Vec::new())
    }
}
