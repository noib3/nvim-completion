use mlua::{Result, Table};

pub struct Api<'a>(pub(crate) Table<'a>);

impl<'a> Api<'a> {
    pub(crate) fn new(vim: Table<'a>) -> Result<Api<'a>> {
        Ok(Api(vim.get::<&str, Table>("api")?))
    }
}
