use async_trait::async_trait;
use serde::Deserialize;

use super::super::completion_source::CompletionSource;
use super::lorems::{LOREMS, LOREM_IPSUM};
use crate::completion::{CompletionItem, Completions};
use crate::cursor::Cursor;

#[derive(Debug, Deserialize)]
pub struct Lipsum {
    pub enable: bool,
}

impl Default for Lipsum {
    fn default() -> Self {
        Lipsum { enable: false }
    }
}

#[async_trait]
impl CompletionSource for Lipsum {
    // Attach to all buffers.
    async fn attach(&self, _bufnr: u32) -> bool {
        true
    }

    async fn complete(&self, cursor: &Cursor) -> Completions {
        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Vec::new();
        }

        LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|&word| CompletionItem {
                details: Some(
                    LOREM_IPSUM.iter().map(|word| word.to_string()).collect(),
                ),
                format: word.to_string(),
                hl_ranges: vec![],
                matched_bytes: word_pre.len() as u32,
                source: "Lipsum",
                text: word.to_string(),
            })
            .collect()
    }
}
