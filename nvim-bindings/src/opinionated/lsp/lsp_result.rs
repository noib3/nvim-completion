use mlua::prelude::LuaError;

pub type LspResult<T> = std::result::Result<T, LspError>;

/// TODO: docs
#[derive(Debug)]
pub enum LspError {
    Lua(LuaError),
    Any(String),
}

impl From<LuaError> for LspError {
    fn from(err: LuaError) -> Self {
        Self::Lua(err)
    }
}
