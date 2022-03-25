use mlua::prelude::{Lua, LuaResult, LuaValue, ToLua};
use serde::Deserialize;

use super::BorderItem;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BorderStyle {
    // Here we need another `BorderString` enum to hold all the string variants
    // ("single", "double", "rounded", etc.). Can be removed if/when
    // https://github.com/serde-rs/serde/issues/1402 is implemented.
    String(BorderString),

    // These variants allow the users to pass a table to customize the
    // characters used in the borders at the corner and edge level, for
    // example:
    // ```lua
    // style = {"a", "b", "c", "d"}
    //
    // style = {
    //   {"a", "FloatBorder"},
    //   {"b", "FloatBorder"},
    //   {"c", "FloatBorder"},
    //   {"d", "FloatBorder"},
    // }
    //
    // style = {
    //   "a",
    //   {"b", "FloatBorder"},
    //   {"c"},
    //   "",
    // }
    // ```
    Array1([BorderItem; 1]),
    Array2([BorderItem; 2]),
    Array4([BorderItem; 4]),
    Array8([BorderItem; 8]),
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

            BorderStyle::Array1(a) => a.to_vec().to_lua(lua),
            BorderStyle::Array2(a) => a.to_vec().to_lua(lua),
            BorderStyle::Array4(a) => a.to_vec().to_lua(lua),
            BorderStyle::Array8(a) => a.to_vec().to_lua(lua),
        }
    }
}
