use std::sync::Arc;

use async_trait::async_trait;
use bindings::opinionated::{Buffer, Neovim};
use mlua::Lua;

use crate::completion_item::Completions;
use crate::completion_context::Cursor;

pub type ShouldAttach = bool;
pub type ShouldRecompute = bool;
pub type Sources = Vec<Arc<tokio::sync::Mutex<dyn CompletionSource>>>;

#[async_trait]
pub trait CompletionSource: std::fmt::Debug + Send + Sync {
    /// Called once when starting the plugin.
    fn setup(&mut self, _lua: &Lua) -> crate::Result<()> {
        Ok(())
    }

    /// Called every time the user enters insert mode.
    fn on_insert_enter(
        &mut self,
        _lua: &Lua,
        _buffer: &Buffer,
    ) -> crate::Result<()> {
        Ok(())
    }

    /// Called on every insert mode edit in an attached buffer. Return `true`
    /// if the source should recompute its completions.
    fn on_edit(
        &mut self,
        _lua: &Lua,
        _buffer: &Buffer,
    ) -> crate::Result<ShouldRecompute> {
        Ok(true)
    }

    /// Called the first time a new buffer is opened. Return `true` if the
    /// source should attach to the buffer.
    fn on_buf_enter(
        &mut self,
        lua: &Lua,
        buffer: &Buffer,
    ) -> crate::Result<ShouldAttach>;

    /// Returns the completion results.
    async fn complete(
        &mut self,
        nvim: &Neovim,
        cursor: &Cursor,
        buffer: &Buffer,
    ) -> crate::Result<Completions>;
}
