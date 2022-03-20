use mlua::prelude::{Lua, LuaResult, LuaValue, ToLua};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BorderSettings {
    #[serde(default)]
    pub enable: bool,

    #[serde(default)]
    pub style: BorderStyle,
}

impl Default for BorderSettings {
    fn default() -> Self {
        BorderSettings {
            enable: bool::default(),
            style: BorderStyle::default(),
        }
    }
}

impl BorderSettings {
    /// Whether the left edge of the border is set (i.e. takes up column).
    pub fn is_left_edge_set(&self) -> bool {
        if !self.enable {
            return false;
        }

        match &self.style {
            BorderStyle::String(s) => match s {
                BorderString::None => false,
                BorderString::Shadow => false,
                _ => true,
            },

            // TODO: implement this
            _ => true,
        }
    }

    /// Whether the right edge of the border is set (i.e. takes up column).
    pub fn _is_right_edge_set(&self) -> bool {
        if !self.enable {
            return false;
        }

        match &self.style {
            BorderStyle::String(s) => match s {
                BorderString::None => false,
                // Technically the right edge is present when the border's
                // style is "shadow", but if it's supposed to be a shadow it
                // looks better if we draw the details over it.
                BorderString::Shadow => false,
                _ => true,
            },

            // TODO: implement this
            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BorderStyle {
    String(BorderString),
    // TODO: allow empty string (easy to do if we take in strings instead of
    // chars, but then you lose some nice typechecking against strings with
    // more than 1 character).
    Array1([char; 1]),
    Array2([char; 2]),
    Array4([char; 4]),
    Array8([char; 8]),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderString {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::String(BorderString::Single)
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

            // TODO: is there a way not to repeat the same exact code 4 times?
            // I can't group them together in the same arm because `a` has
            // different types in each (`[char; 1]` vs `[char; 2]` vs etc..).
            BorderStyle::Array1(a) => a
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .to_lua(lua),

            BorderStyle::Array2(a) => a
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .to_lua(lua),

            BorderStyle::Array4(a) => a
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .to_lua(lua),

            BorderStyle::Array8(a) => a
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .to_lua(lua),
        }
    }
}
