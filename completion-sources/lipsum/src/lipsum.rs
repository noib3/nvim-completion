use async_trait::async_trait;
use nvim_completion_core::{
    Buffer,
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
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

    async fn should_attach(
        &self,
        _buf: &Buffer,
        _config: &Config,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn complete(
        &self,
        _buf: &Buffer,
        _ctx: &CompletionContext,
        _config: &Config,
    ) -> Result<Vec<CompletionItem>> {
        let completions = super::WORDS
            .iter()
            .map(|word| CompletionItemBuilder::new(*word).build())
            .collect::<Vec<_>>();

        Ok(completions)
    }
}
