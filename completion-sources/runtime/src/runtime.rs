use async_trait::async_trait;
use nvim_completion_core::{
    Buffer,
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
    RuntimeSource,
};
use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(RuntimeSource)]
pub struct Runtime;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {}

#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct Error(&'static str);

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl CompletionSource for Runtime {
    const NAME: &'static str = "runtime";

    type Config = Config;

    type Error = Error;

    async fn complete(
        &self,
        _buf: &Buffer,
        _ctx: &CompletionContext,
        _config: &Config,
    ) -> Result<Vec<CompletionItem>> {
        Ok(vec![CompletionItemBuilder::new("from dynamic").build()])
    }
}
