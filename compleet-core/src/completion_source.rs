use async_trait::async_trait;
use nvim_oxi::{api::Buffer, Function, Object};
use serde::Deserialize;

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

    async fn complete(&self) -> Vec<String>;

    #[inline]
    fn api(&self) -> Object {
        Object::nil()
    }
}
