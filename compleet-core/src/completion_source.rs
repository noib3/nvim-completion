use async_trait::async_trait;

#[async_trait]
pub trait CompletionSource: Send + Sync {}
