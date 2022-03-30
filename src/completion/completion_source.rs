use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use mlua::prelude::LuaResult;
use neovim::Api;

use super::{Completions, Cursor};

pub type Sources = Vec<Arc<dyn CompletionSource>>;

#[async_trait]
pub trait CompletionSource: Debug + Send + Sync {
    /// Decides whether to attach the source to a buffer.
    fn attach(&self, api: &Api, bufnr: u32) -> LuaResult<bool>;

    /// The function used to get completion results. Takes in an `api` field
    /// (providing the functionality of `vim.api`) and the current cursor
    /// position.
    async fn complete(&self, cursor: &Cursor) -> Completions;
}
