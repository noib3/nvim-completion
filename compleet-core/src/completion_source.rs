use async_trait::async_trait;
use nvim_oxi::Object;

// use serde::Deserialize;
use crate::{CompletionContext, CompletionItem, Result};

pub(crate) type SourceId = &'static str;

#[async_trait]
pub trait CompletionSource: Send + Sync + 'static {
    /// The name of the completion source.
    fn name(&self) -> &'static str;

    /// TODO: docs
    #[inline]
    fn api(&self) -> Object {
        Object::nil()
    }

    /// TODO: docs
    async fn should_attach(&self, _buf: &crate::Buffer) -> Result<bool> {
        Ok(true)
    }

    /// TODO: docs
    async fn complete(
        &self,
        buf: &crate::Buffer,
        ctx: &CompletionContext,
    ) -> Result<Vec<CompletionItem>>;
}
