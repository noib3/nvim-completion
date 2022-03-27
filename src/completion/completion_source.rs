use std::fmt;

use mlua::prelude::LuaResult;
use neovim::Api;

use super::{CompletionItem, Cursor};

pub trait CompletionSource: fmt::Debug {
    /// Whether the source should be enabled by default or be opt-in by the
    /// user.
    fn enable(&self) -> bool;

    /// Whether to attach the source to a buffer.
    fn attach(&self, api: &Api, bufnr: u32) -> LuaResult<bool>;

    /// .
    fn complete(
        &self,
        api: &Api,
        cursor: &Cursor,
    ) -> LuaResult<Vec<CompletionItem>>;
}
