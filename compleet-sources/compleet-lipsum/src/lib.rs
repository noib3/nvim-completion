use async_trait::async_trait;
use compleet_core as compleet;

pub struct CompleetLipsum;

#[async_trait]
impl compleet::CompletionSource for CompleetLipsum {
    #[inline]
    fn name(&self) -> &'static str {
        "lipsum"
    }

    async fn complete(&self) -> Vec<String> {
        Vec::new()
    }
}
