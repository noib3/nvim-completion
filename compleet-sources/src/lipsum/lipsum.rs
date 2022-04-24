use async_trait::async_trait;
use bindings::opinionated::Neovim;

use super::{
    lorems::{LOREMS, LOREM_IPSUM},
    LipsumConfig,
};
use crate::prelude::{
    CompletionItem,
    CompletionSource,
    Completions,
    Cursor,
    Result,
};

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

    async fn complete(
        &self,
        _nvim: &Neovim,
        cursor: &Cursor,
    ) -> Result<Completions> {
        // // Simulate a slow source, this shouldn't block.
        // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        Ok(LOREMS
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
            .collect())
    }
}
