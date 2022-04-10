use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;

use crate::completion::Completions;
use crate::cursor::Cursor;

pub type Sources = Vec<Arc<dyn CompletionSource>>;

#[async_trait]
pub trait CompletionSource: Debug + Send + Sync {
    /// Decides whether to attach the source to a buffer.
    async fn attach(&self, bufnr: u32) -> bool;

    /// Returns the completion results.
    async fn complete(&self, cursor: &Cursor) -> Completions;
}
