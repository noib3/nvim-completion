use async_trait::async_trait;
use compleet_core::{
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
    Result,
};

pub struct CompleetLipsum;

#[async_trait]
impl CompletionSource for CompleetLipsum {
    fn name(&self) -> &'static str {
        "lipsum"
    }

    async fn complete(
        &self,
        _ctx: &CompletionContext,
    ) -> Result<Vec<CompletionItem>> {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let completions = super::WORDS
            .iter()
            .map(|word| CompletionItemBuilder::new(*word).build())
            .collect::<Vec<_>>();

        Ok(completions)
    }
}
