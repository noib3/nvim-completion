use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use bindings::opinionated::Neovim;
use mlua::prelude::{Lua, LuaResult};
use tokio::sync::Mutex;

use crate::prelude::{Completions, Cursor, Result};

pub type Sources = Vec<Arc<Mutex<dyn CompletionSource>>>;

#[async_trait]
pub trait CompletionSource: Debug + Send + Sync {
    /// Called once when starting the plugin.
    fn setup(&mut self, _lua: &Lua) -> LuaResult<()> {
        Ok(())
    }

    /// Called the first time a buffer is opened, return `true` if the source
    /// should attach to the buffer.
    // async fn attach(&mut self, nvim: &Neovim, bufnr: u16) -> bool;
    fn attach(&mut self, lua: &Lua, bufnr: u16) -> LuaResult<bool>;

    /// Returns the completion results.
    async fn complete(
        &self,
        nvim: &Neovim,
        cursor: &Cursor,
        bufnr: u16,
    ) -> Result<Completions>;
}
