use async_trait::async_trait;
use completion_types::{
    CompletionItem,
    CompletionList,
    CompletionSource,
    Document,
    Position,
};
use serde::Deserialize;
use thiserror::Error as ThisError;

pub struct Lipsum;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {}

#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct Error(&'static str);

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl CompletionSource for Lipsum {
    const NAME: &'static str = "lipsum";

    type Config = Config;

    type Error = Error;

    async fn enable(&self, _doc: &Document, _config: &Config) -> Result<bool> {
        Ok(true)
    }

    async fn trigger_characters(
        &self,
        _doc: &Document,
        _config: &Config,
    ) -> Result<Vec<char>> {
        Ok(Vec::new())
    }

    async fn complete(
        &self,
        _doc: &Document,
        _pos: &Position,
        _config: &Config,
    ) -> Result<CompletionList> {
        let completions = super::WORDS
            .iter()
            .map(|word| CompletionItem::builder().text(*word).build())
            .collect::<Vec<_>>();

        Ok(CompletionList { items: completions, is_complete: true })
    }
}
