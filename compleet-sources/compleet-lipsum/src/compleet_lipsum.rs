use async_trait::async_trait;
use compleet_core::{CompletionContext, CompletionItem, CompletionSource};

pub struct CompleetLipsum;

#[async_trait]
impl CompletionSource for CompleetLipsum {
    #[inline]
    fn name(&self) -> &'static str {
        "lipsum"
    }

    async fn complete(&self, ctx: &CompletionContext) -> Vec<CompletionItem> {
        vec![CompletionItem::new(format!(
            "{} received {}",
            self.name(),
            ctx.ch()
        ))]
    }
}
