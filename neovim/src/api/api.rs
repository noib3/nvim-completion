use mlua::{Result, Table};

pub struct Api<'a>(pub(crate) Table<'a>);

impl<'a> Api<'a> {
    pub(crate) fn new(vim: Table<'a>) -> Result<Api<'a>> {
        Ok(Api(vim.get::<&str, Table>("api")?))
    }
}

/// TODO: docs
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}
