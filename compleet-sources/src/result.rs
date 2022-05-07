use std::fmt;

use bindings::opinionated::lsp;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lsp(lsp::Error),
    Lua(mlua::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Lsp(err) => write!(f, "{}", err),
            Lua(err) => write!(f, "{}", err),
        }
    }
}

impl From<lsp::Error> for Error {
    fn from(err: lsp::Error) -> Self {
        Self::Lsp(err)
    }
}

impl From<mlua::Error> for Error {
    fn from(err: mlua::Error) -> Self {
        Self::Lua(err)
    }
}
