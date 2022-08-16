use async_trait::async_trait;
use compleet_core::{
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
};

pub struct CompleetLipsum;

#[async_trait]
impl CompletionSource for CompleetLipsum {
    fn name(&self) -> &'static str {
        "lipsum"
    }

    async fn complete(&self, _ctx: &CompletionContext) -> Vec<CompletionItem> {
        super::WORDS
            .iter()
            .map(|word| CompletionItemBuilder::new(*word).build())
            .collect::<Vec<_>>()
    }
}
