use async_trait::async_trait;
use serde::Deserialize;

use super::super::completion_source::CompletionSource;
use crate::{
    completion::{CompletionItem, Completions},
    cursor::Cursor,
};

#[derive(Debug, Deserialize)]
pub struct Lsp {
    pub enable: bool,
    pub test: String,
}

impl Default for Lsp {
    fn default() -> Self {
        Lsp { enable: false, test: "Default".into() }
    }
}

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
                format: format!(" {} ", self.test),
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
