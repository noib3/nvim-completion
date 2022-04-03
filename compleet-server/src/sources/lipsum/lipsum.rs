use async_trait::async_trait;
use serde::Deserialize;

use super::lorems::{LOREMS, LOREM_IPSUM};
use crate::completion::{Completion, Completions};
use crate::cursor::Cursor;
use crate::source::Source;

#[derive(Debug, Deserialize)]
pub struct Lipsum {
    pub enable: bool,
}

impl Default for Lipsum {
    fn default() -> Self { Lipsum { enable: false } }
}

#[async_trait]
impl Source for Lipsum {
    // Attach to all buffers.
    fn attach(&self, _bufnr: u32) -> bool { true }

    async fn complete(&self, cursor: &Cursor) -> Completions {
        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Vec::new();
        }

        // Simulate a slow source, this shouldn't block.
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        // std::thread::sleep(std::time::Duration::from_secs(5));

        LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|&word| Completion {
                details: Some(
                    LOREM_IPSUM.iter().map(|word| word.to_string()).collect(),
                ),
                format: format!(" {}", word),
                hl_ranges: vec![(
                    1..word_pre.len() + 1,
                    "CompleetMenuMatchingChars",
                )],
                matched_bytes: word_pre.len() as u32,
                source: "Lipsum",
                text: word.to_string(),
            })
            .collect()
    }
}
