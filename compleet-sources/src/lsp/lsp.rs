use async_trait::async_trait;
use bindings::opinionated::{
    lsp::{
        protocol::{CompletionParams, CompletionResponse},
        LspMethod,
    },
    Neovim,
};

use super::LspConfig;
use crate::prelude::{
    CompletionItem,
    CompletionSource,
    Completions,
    Cursor,
    Result,
};

#[derive(Debug, Default)]
pub struct Lsp {
    _config: LspConfig,
}

impl From<LspConfig> for Lsp {
    fn from(_config: LspConfig) -> Self {
        Self { _config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lsp {
    async fn attach(&mut self, _nvim: &Neovim, _bufnr: u16) -> bool {
        // TODO: check if buffer has any LSPs available.
        // vim.lsp.buf_is_attached
        true
    }

    async fn complete(
        &self,
        nvim: &Neovim,
        cursor: &Cursor,
    ) -> Result<Completions> {
        let client = match nvim.lsp_buf_get_clients(0).await {
            v if v.is_empty() => return Ok(Vec::new()),
            v => v.into_iter().nth(0).unwrap(),
        };

        let method = LspMethod::Completion(CompletionParams::new(
            nvim.api_buf_get_name(0).await,
            cursor.row as u32,
            cursor.bytes as u32,
        ));

        let items = match client.request(method, 0).await? {
            CompletionResponse::CompletionList(list) => list.items,
            CompletionResponse::CompletionItems(items) => items,
        };

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        Ok(items
            .into_iter()
            .filter(|item| {
                item.label.starts_with(word_pre) && item.label != word_pre
            })
            .map(|item| {
                let mut item: CompletionItem = item.into();
                item.matched_bytes = vec![0..word_pre.len()];
                item.matched_prefix = word_pre.len() as u16;
                item
            })
            .collect::<Completions>())
    }
}
