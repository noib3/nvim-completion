use async_trait::async_trait;
use nvim_completion_core::{
    Buffer,
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
    Result,
};
use serde::Deserialize;

pub struct Lipsum;

#[derive(Deserialize)]
pub struct Config {}

#[async_trait]
impl CompletionSource for Lipsum {
    const NAME: &'static str = "lipsum";

    type Config = Config;

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
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let completions = super::WORDS
            .iter()
            .map(|word| CompletionItemBuilder::new(*word).build())
            .collect::<Vec<_>>();

        Ok(completions)
    }
}
