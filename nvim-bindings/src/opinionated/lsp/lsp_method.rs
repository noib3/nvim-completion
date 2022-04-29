use mlua::prelude::{
    Lua,
    LuaResult,
    LuaSerdeExt,
    LuaSerializeOptions,
    LuaValue,
    ToLua,
};

use super::protocol::CompletionParams;

const SERIALIZE_OPTIONS: LuaSerializeOptions = LuaSerializeOptions::new()
    .serialize_none_to_null(false)
    .serialize_unit_to_null(false);

impl<'lua> ToLua<'lua> for CompletionParams {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.to_value_with(&self, SERIALIZE_OPTIONS)
    }
}
