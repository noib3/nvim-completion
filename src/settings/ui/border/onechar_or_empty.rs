use std::fmt;

use mlua::prelude::{Lua, LuaResult, LuaValue, ToLua};
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize,
};

#[derive(Debug, Clone)]
pub struct OnecharOrEmpty(pub String);

impl<'de> Deserialize<'de> for OnecharOrEmpty {
    fn deserialize<D>(deserializer: D) -> Result<OnecharOrEmpty, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnecharVisitor;

        impl<'de> Visitor<'de> for OnecharVisitor {
            type Value = OnecharOrEmpty;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                formatter
                    .write_str("either a single character or an empty string")
            }

            fn visit_str<E: de::Error>(
                self,
                value: &str,
            ) -> Result<Self::Value, E> {
                match value.chars().count() {
                    0 | 1 => Ok(OnecharOrEmpty(value.to_string())),
                    _ => Err(E::custom("no more than 1 character!")),
                }
            }
        }

        deserializer.deserialize_str(OnecharVisitor)
    }
}

impl<'lua> ToLua<'lua> for OnecharOrEmpty {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.0.to_lua(lua)
    }
}
