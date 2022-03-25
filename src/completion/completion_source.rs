use std::fmt;

use mlua::prelude::LuaResult;
use neovim::Api;

use super::{CompletionItem, Cursor};

pub trait CompletionSource: fmt::Debug {
    fn complete(
        &self,
        api: &Api,
        cursor: &Cursor,
    ) -> LuaResult<Vec<CompletionItem>>;
}
