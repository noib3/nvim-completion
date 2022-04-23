use mlua::prelude::{
    Lua,
    LuaResult,
    LuaSerdeExt,
    LuaSerializeOptions,
    LuaValue,
};

use super::protocol::CompletionParams;

const SERIALIZE_OPTIONS: LuaSerializeOptions = LuaSerializeOptions::new()
    .serialize_none_to_null(false)
    .serialize_unit_to_null(false);

/// Subset of `:h lsp-method` relevant to code completion.
pub enum LspMethod {
    Completion(CompletionParams),
}

impl LspMethod {
    pub fn expand(self, lua: &Lua) -> LuaResult<(&'static str, LuaValue<'_>)> {
        use LspMethod::*;

        match self {
            Completion(params) => Ok((
                "textDocument/completion",
                lua.to_value_with(&params, SERIALIZE_OPTIONS)?,
            )),
        }
    }
}
