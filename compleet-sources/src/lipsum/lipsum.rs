use async_trait::async_trait;

use super::{
    lorems::{LOREMS, LOREM_IPSUM},
    LipsumConfig,
};
use common::{CompletionItem, CompletionSource, Completions, Cursor, Neovim};

#[derive(Debug, Default)]
pub struct Lipsum {
    _config: LipsumConfig,
}

impl From<LipsumConfig> for Lipsum {
    fn from(_config: LipsumConfig) -> Self {
        Self { _config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lipsum {
    async fn attach(&mut self, _nvim: &Neovim, _bufnr: u16) -> bool {
        true
    }

    async fn complete(&self, _nvim: &Neovim, cursor: &Cursor) -> Completions {
        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Vec::new();
        }

        LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|word| CompletionItem {
                details: Some(
                    LOREM_IPSUM.iter().map(|word| word.to_string()).collect(),
                ),
                format: format!(" {word} "),
                matched_bytes: vec![0..word_pre.len()],
                matched_prefix: word_pre.len() as u16,
                source: "Lipsum",
                text: word.to_string(),
            })
            .collect()
    }
}
