use std::fmt;

use bindings::opinionated::lsp::LspError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lsp(LspError),
}

impl From<LspError> for Error {
    fn from(err: LspError) -> Self {
        Self::Lsp(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Lsp(err) => write!(f, "{}", err),
        }
    }
}
