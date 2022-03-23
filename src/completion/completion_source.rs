use super::{CompletionItem, Cursor};
use mlua::prelude::LuaResult;
use neovim::Api;

pub trait CompletionSource {
    fn complete(
        &self,
        api: &Api,
        cursor: &Cursor,
    ) -> LuaResult<Vec<CompletionItem>>;
}
