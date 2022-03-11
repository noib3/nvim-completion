use mlua::{Function, Result, Table};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_open_win`.
    ///
    /// Opens a new window. Used to open floating and external windows.
    ///
    /// # Arguments
    ///
    /// * `bufnr`   Buffer to display, or 0 for current buffer.
    /// * `enter`   Whether to enter the newly created window, making it the current window.
    /// * `config`  Map defining the window configuration. See `:h nvim_open_win` for details.
    pub fn open_win(
        &self,
        bufnr: usize,
        enter: bool,
        config: Table,
    ) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_open_win")?
            .call::<_, usize>((bufnr, enter, config))?)
    }
}
