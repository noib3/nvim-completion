use mlua::{Function, Result, Table};

pub struct Keymap<'a>(Table<'a>);

impl<'a> Keymap<'a> {
    pub(crate) fn new(vim: Table<'a>) -> Result<Keymap<'a>> {
        Ok(Keymap(vim.get::<&str, Table>("keymap")?))
    }
}

impl<'a> Keymap<'a> {
    // TODO: make `mode` generic over `&str` and `&[&str]`, `rhs` generic over
    // `&str` and `Function`.
    /// Binding to `vim.keymap.set`
    ///
    /// Adds a new mapping.
    pub fn set(
        &self,
        mode: &str,
        lhs: &str,
        rhs: Function,
        opts: Option<Table>,
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("set")?
            .call::<_, ()>((mode, lhs, rhs, opts))?)
    }
}
