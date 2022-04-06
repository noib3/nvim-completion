use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;

use super::completion::Completions;
use super::cursor::Cursor;

pub type Sources = Vec<Arc<dyn Source>>;

#[async_trait]
pub trait Source: Debug + Send + Sync {
    /// Decides whether to attach the source to a buffer.
    async fn attach(&self, bufnr: u32) -> bool;

    /// The function used to get completion results. Takes in an `api` field
    /// (providing the functionality of `vim.api`) and the current cursor
    /// position.
    async fn complete(&self, cursor: &Cursor) -> Completions;
}
