use async_trait::async_trait;

use super::LspConfig;
use common::{CompletionItem, CompletionSource, Completions, Cursor, Neovim};

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
        true
    }

    async fn complete(&self, nvim: &Neovim, cursor: &Cursor) -> Completions {
        let attached = nvim.lsp_buf_get_clients(0).await;

        // Simulate a slow source, this shouldn't block.
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let word_pre = cursor.word_pre();
        if word_pre.is_empty() {
            return Vec::new();
        }

        let test = &self.config.test;
        if test.starts_with(word_pre) && test != word_pre {
            vec![CompletionItem {
                details: None,
                format: format!(" {test} - {attached} "),
                matched_bytes: vec![0..word_pre.len()],
                matched_prefix: word_pre.len() as u16,
                source: "Lsp",
                text: test.clone(),
            }]
        } else {
            Vec::new()
        }
    }
}
