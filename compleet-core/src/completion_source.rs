use async_trait::async_trait;
use nvim_oxi::{api::Buffer, Function, Object};
use serde::Deserialize;

use crate::{CompletionContext, CompletionItem};

#[derive(Deserialize)]
#[serde(untagged)]
enum EnableCompletion {
    Ready(bool),
    Maybe(Function<Buffer, bool>),
}

#[async_trait]
pub trait CompletionSource: Send + Sync + 'static {
    /// The name of the completion source.
    fn name(&self) -> &'static str;

    async fn complete(&self, ctx: &CompletionContext) -> Vec<CompletionItem>;

    #[inline]
    fn api(&self) -> Object {
        Object::nil()
    }
}
