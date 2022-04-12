use async_trait::async_trait;
use serde::Deserialize;

use super::super::completion_source::CompletionSource;
use crate::completion::{CompletionItem, Completions};
use crate::cursor::Cursor;

#[derive(Debug, Deserialize)]
pub struct Lsp {
    pub enable: bool,
    pub test: String,
}

impl Default for Lsp {
    fn default() -> Self {
        Lsp {
            enable: false,
            test: "Default".into(),
        }
    }
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
    async fn attach(&self, _bufnr: u32) -> bool {
        // let clients = nvim.lsp.buf_get_clients(bufnr)?;
        true
    }

    async fn complete(&self, cursor: &Cursor) -> Completions {
        // Simulate a slow source, this shouldn't block.
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let word_pre = cursor.word_pre();
        if word_pre.is_empty() {
            return Vec::new();
        }

        if self.test.starts_with(word_pre) && self.test != word_pre {
            vec![CompletionItem {
                details: None,
                format: format!(" {word_pre} "),
                matched_bytes: vec![0..word_pre.len()],
                matched_prefix: word_pre.len() as u32,
                source: "Lsp",
                text: self.test.clone(),
            }]
        } else {
            Vec::new()
        }
    }
}
