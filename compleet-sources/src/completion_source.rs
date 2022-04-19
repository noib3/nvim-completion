use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use mlua::prelude::{Lua, LuaResult};
use tokio::sync::Mutex;

use crate::prelude::{Completions, Cursor};

pub type Sources = Vec<Arc<Mutex<dyn CompletionSource>>>;

#[async_trait]
pub trait CompletionSource: Debug + Send + Sync {
    /// Decides whether to attach the source to a buffer.
    fn attach(&mut self, lua: &Lua, bufnr: u16) -> LuaResult<bool>;

    /// Returns the completion results.
    async fn complete(&self, cursor: &Cursor) -> Completions;
}
