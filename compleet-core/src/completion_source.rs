use async_trait::async_trait;
use nvim_oxi::{api::Buffer, Object};

// use serde::Deserialize;
use crate::{CompletionContext, CompletionItem, Result};

// #[derive(Deserialize)]
// #[serde(untagged)]
// enum EnableCompletion {
//     Ready(bool),
//     Maybe(Function<Buffer, bool>),
// }

#[async_trait]
pub trait CompletionSource: Send + Sync + 'static {
    /// The name of the completion source.
    fn name(&self) -> &'static str;

    /// TODO: docs
    async fn complete(
        &self,
        ctx: &CompletionContext,
    ) -> Result<Vec<CompletionItem>>;

    /// TODO: docs
    async fn should_attach(&self, buf: &Buffer) -> Result<bool> {
        Ok(true)
    }

    /// TODO: docs
    #[inline]
    fn api(&self) -> Object {
        Object::nil()
    }
}
