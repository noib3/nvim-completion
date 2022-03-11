use mlua::{Function, Result, Table};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_create_augroup`.
    ///
    /// Creates or gets an augroup. Returns the id to be used in
    /// `Nvim::del_augroup_by_id`.
    ///
    /// # Arguments
    ///
    /// * `name`  The name of the augroup to create
    /// * `opts`  Optional parameters. See `:h nvim_create_augroup` for  details
    pub fn create_augroup(&self, name: &str, opts: Table) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_create_augroup")?
            .call::<_, usize>((name, opts))?)
    }

    /// Binding to `vim.api.nvim_create_autocmd`.
    ///
    /// Creates an autocmd. Returns an id to be used in `Nvim::del_autocmd`.
    ///
    /// # Arguments
    ///
    /// * `events`  A slice of strings reprenting event names
    /// * `opts`    Optional parameters. See `:h nvim_create_autocmd` for  details
    pub fn create_autocmd(
        &self,
        events: &[&str],
        opts: Table,
    ) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_create_autocmd")?
            .call::<_, usize>((events, opts))?)
    }
}
