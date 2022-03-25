use mlua::prelude::{Lua, LuaResult, LuaValue, ToLua};

use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum BorderItem {
    Char(String),
    Tuple((String, Option<String>)),
}

impl<'de> Deserialize<'de> for BorderItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemVisitor;

        impl<'de> Visitor<'de> for ItemVisitor {
            type Value = BorderItem;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                write!(formatter, "a string or a tuple")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match s.chars().count() {
                    0 | 1 => Ok(BorderItem::Char(s.into())),
                    _ => Err(E::invalid_length(
                        s.chars().count(),
                        &"either 0 or 1 characters",
                    )),
                }
            }

            fn visit_seq<S>(
                self,
                mut visitor: S,
            ) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                let len = visitor
                    .size_hint()
                    .expect("couldn't determine item length");

                if len == 0 || len > 2 {
                    return Err(de::Error::invalid_length(
                        len,
                        &"each tuple should have either 1 or 2 elements",
                    ));
                }

                // Deserializing the character
                let c = visitor
                    .next_element::<String>()?
                    .expect("Already checked that len is > 1");

                if c.chars().count() > 2 {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(&c),
                        &"either 0 or 1 characters",
                    ));
                }

                // Deserializing the highlight group
                let hl_group = visitor.next_element::<String>()?;

                Ok(BorderItem::Tuple((c, hl_group)))
            }
        }

        deserializer.deserialize_any(ItemVisitor)
    }
}

impl<'lua> ToLua<'lua> for BorderItem {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        match self {
            Self::Char(c) => c.to_lua(lua),
            Self::Tuple((c, maybe_hl)) => {
                Ok(LuaValue::Table(if let Some(hl) = maybe_hl {
                    lua.create_sequence_from([c, hl])?
                } else {
                    lua.create_sequence_from([c])?
                }))
            },
        }
    }
}
