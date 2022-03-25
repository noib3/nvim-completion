use mlua::prelude::{Lua, LuaResult, LuaValue, ToLua};
use serde::Deserialize;

use super::OnecharOrEmpty;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BorderStyle {
    // Here we need another `BorderString` enum to hold all the string variants
    // ("single", "double", "rounded", etc.). Can be removed if/when
    // https://github.com/serde-rs/serde/issues/1402 is implemented.
    String(BorderString),

    // TODO: should actually be
    // Array1([OnecharOrEmpty | (OnecharOrEmpty, Option<String>); 1]),
    // Array2([OnecharOrEmpty | (OnecharOrEmpty, Option<String>); 2]),
    // Array4([OnecharOrEmpty | (OnecharOrEmpty, Option<String>); 4]),
    // Array8([OnecharOrEmpty | (OnecharOrEmpty, Option<String>); 8]),
    // Add an enum `BorderItem`?

    // These variants allow the users to pass a table to customize the
    // characters used in the borders at the corner and edge level, for
    // example:
    // ```lua
    // style = {"a", "b", "c", "d"}
    // ```
    Array1([OnecharOrEmpty; 1]),
    Array2([OnecharOrEmpty; 2]),
    Array4([OnecharOrEmpty; 4]),
    Array8([OnecharOrEmpty; 8]),

    // Same as before, but now every item in the table is a tuple instead of a
    // string, with the second element specifying a highlight group, for
    // example
    // ```lua
    // style = {
    //   {"a", "FloatBorder"},
    //   {"b", "FloatBorder"},
    //   {"c", "FloatBorder"},
    //   {"d", "FloatBorder"},
    // }
    // ```
    Array1WithHlgroup([(OnecharOrEmpty, String); 1]),
    Array2WithHlgroup([(OnecharOrEmpty, String); 2]),
    Array4WithHlgroup([(OnecharOrEmpty, String); 4]),
    Array8WithHlgroup([(OnecharOrEmpty, String); 8]),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderString {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
}

impl BorderStyle {
    pub fn has_top_edge(&self) -> bool {
        match self {
            Self::String(s) => match s {
                BorderString::None => false,
                BorderString::Shadow => false,
                _ => true,
            },

            // TODO
            _ => true,
        }
    }

    pub fn has_bottom_edge(&self) -> bool {
        match self {
            Self::String(s) => match s {
                BorderString::None => false,
                BorderString::Shadow => false,
                _ => true,
            },

            // TODO
            _ => true,
        }
    }

    pub fn has_left_edge(&self) -> bool {
        match self {
            Self::String(s) => match s {
                BorderString::None => false,
                BorderString::Shadow => false,
                _ => true,
            },

            // TODO
            _ => true,
        }
    }

    pub fn has_right_edge(&self) -> bool {
        match self {
            Self::String(s) => match s {
                BorderString::None => false,
                BorderString::Shadow => false,
                _ => true,
            },

            // TODO
            _ => true,
        }
    }
}

impl BorderStyle {
    pub fn to_lua<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        match self {
            BorderStyle::String(s) => match s {
                BorderString::None => "none".to_lua(lua),
                BorderString::Single => "single".to_lua(lua),
                BorderString::Double => "double".to_lua(lua),
                BorderString::Rounded => "rounded".to_lua(lua),
                BorderString::Solid => "solid".to_lua(lua),
                BorderString::Shadow => "shadow".to_lua(lua),
            },

            // Well this is annoying..
            BorderStyle::Array1(a) => a.to_vec().to_lua(lua),
            BorderStyle::Array2(a) => a.to_vec().to_lua(lua),
            BorderStyle::Array4(a) => a.to_vec().to_lua(lua),
            BorderStyle::Array8(a) => a.to_vec().to_lua(lua),

            // ..and this even more so!
            BorderStyle::Array1WithHlgroup(a) => a
                .to_vec()
                .into_iter()
                .map(|(c, hl)| [c.0, hl])
                .collect::<Vec<[String; 2]>>()
                .to_lua(lua),

            BorderStyle::Array2WithHlgroup(a) => a
                .to_vec()
                .into_iter()
                .map(|(c, hl)| [c.0, hl])
                .collect::<Vec<[String; 2]>>()
                .to_lua(lua),

            BorderStyle::Array4WithHlgroup(a) => a
                .to_vec()
                .into_iter()
                .map(|(c, hl)| [c.0, hl])
                .collect::<Vec<[String; 2]>>()
                .to_lua(lua),

            BorderStyle::Array8WithHlgroup(a) => a
                .to_vec()
                .into_iter()
                .map(|(c, hl)| [c.0, hl])
                .collect::<Vec<[String; 2]>>()
                .to_lua(lua),
        }
    }
}
