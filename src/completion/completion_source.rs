use std::fmt::Debug;

use mlua::prelude::LuaResult;
use neovim::Api;

// use serde::Deserialize;
use super::{CompletionItem, Cursor};

pub trait CompletionSource: Debug /* + Default + Deserialize<'_> */ {
    /// Whether to attach the source to a buffer.
    fn attach(&self, api: &Api, bufnr: u32) -> LuaResult<bool>;

    /// The function used to get completion results. Takes in an `api` field
    /// (providing the functionality of `vim.api`) and the current cursor
    /// position.
    fn complete(
        &self,
        api: &Api,
        cursor: &Cursor,
    ) -> LuaResult<Vec<CompletionItem>>;
}
