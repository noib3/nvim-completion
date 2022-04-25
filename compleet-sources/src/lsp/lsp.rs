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
    config: LspConfig,
}

impl From<LspConfig> for Lsp {
    fn from(config: LspConfig) -> Self {
        Self { config, ..Default::default() }
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

        let num = match client.request(method, 0).await? {
            CompletionResponse::CompletionList(list) => list.items.len(),
            CompletionResponse::CompletionItems(items) => items.len(),
        };

        let word_pre = cursor.word_pre();
        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        let test = &self.config.test;
        if test.starts_with(word_pre) && test != word_pre {
            Ok(vec![CompletionItem {
                details: None,
                format: format!(" {test} - {} ", num),
                matched_bytes: vec![0..word_pre.len()],
                matched_prefix: word_pre.len() as u16,
                source: "Lsp",
                text: test.clone(),
            }])
        } else {
            Ok(Vec::new())
        }
    }
}
