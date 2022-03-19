use mlua::{FromLua, Function, Result, Table, ToLua};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_win_close`.
    ///
    /// Closes the window.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    /// * `force`  Whether to behave like `:close!`.
    pub fn win_close(&self, winid: u32, force: bool) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_win_close")?
            .call((winid, force))
    }

    /// Binding to `vim.api.nvim_win_get_cursor`
    ///
    /// Returns the (1,0)-indexed cursor position as a tuple.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    pub fn win_get_cursor(&self, winid: u32) -> Result<(u32, u32)> {
        let position = self
            .0
            .get::<&str, Function>("nvim_win_get_cursor")?
            .call::<_, Table>(winid)?;

        Ok((position.get(1)?, position.get(2)?))
    }

    /// Binding to `vim.api.nvim_win_get_option`
    ///
    /// Gets a window option value.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    /// * `name`   Option name.
    /// * `value`  Option value.
    pub fn win_get_option<V: FromLua<'a>>(
        &self,
        winid: u32,
        name: &str,
    ) -> Result<V> {
        self.0
            .get::<&str, Function>("nvim_win_get_option")?
            .call((winid, name))
    }

    /// Binding to `vim.api.nvim_win_get_width`
    ///
    /// Returns the window width as a count of columns.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    pub fn win_get_width(&self, winid: u32) -> Result<u32> {
        self.0
            .get::<&str, Function>("nvim_win_get_width")?
            .call(winid)
    }

    /// Binding to `vim.api.nvim_win_get_width`
    ///
    /// Returns the window height as a count of rows.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    pub fn win_get_height(&self, winid: u32) -> Result<u32> {
        self.0
            .get::<&str, Function>("nvim_win_get_height")?
            .call(winid)
    }

    /// Binding to `vim.api.nvim_win_hide`.
    ///
    /// Closes the window and hides the buffer it contains.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    pub fn win_hide(&self, winid: u32) -> Result<()> {
        self.0.get::<&str, Function>("nvim_win_hide")?.call(winid)
    }

    /// Binding to `vim.api.nvim_win_set_cursor`
    ///
    /// Sets the (1,0)-indexed cursor position in the window.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    /// * `row`    Row number (1-indexed).
    /// * `col`    Column number (0-indexed).
    pub fn win_set_cursor(
        &self,
        winid: u32,
        row: u32,
        col: u32,
    ) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_win_set_cursor")?
            .call((winid, [row, col]))
    }

    /// Binding to `vim.api.nvim_win_set_option`
    ///
    /// Sets a window option value. Passing `None` as a value deletes the
    /// option (only works if there's a global fallback).
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    /// * `name`   Option name.
    /// * `value`  Option value.
    pub fn win_set_option<V: ToLua<'a>>(
        &self,
        winid: u32,
        name: &str,
        value: V,
    ) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_win_set_option")?
            .call((winid, name, value))
    }
}
