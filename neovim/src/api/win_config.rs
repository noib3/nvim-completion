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

    /// Binding to `vim.api.nvim_win_get_config`.
    ///
    /// Gets the window configuration.
    ///
    /// # Arguments
    ///
    /// * `winid`   Window handle, or 0 for current buffer.
    pub fn win_get_config(&self, winid: usize) -> Result<Table> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_get_config")?
            .call::<_, Table>(winid)?)
    }

    /// Binding to `vim.api.nvim_win_set_config`.
    ///
    /// Configures window layout. Currently only for floating and external
    /// windows (including changing a split window to those layouts).
    ///
    /// # Arguments
    ///
    /// * `winid`   Window handle, or 0 for current buffer.
    /// * `config`  Map defining the window configuration. See `:h nvim_open_win` for details.
    pub fn win_set_config(&self, winid: usize, config: Table) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_set_config")?
            .call::<_, ()>((winid, config))?)
    }
}
