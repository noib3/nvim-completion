use mlua::{Function, Result, Table, ToLua};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_get_cursor`
    ///
    /// Returns the (1,0)-indexed cursor position as a tuple.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    pub fn win_get_cursor(&self, winid: usize) -> Result<(usize, usize)> {
        let position = self
            .0
            .get::<&str, Function>("nvim_win_get_cursor")?
            .call::<_, Table>(winid)?;

        Ok((position.get(1)?, position.get(2)?))
    }

    /// Binding to `vim.api.nvim_win_hide`.
    ///
    /// Closes the window and hides the buffer it contains.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window.
    pub fn win_hide(&self, winid: usize) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_hide")?
            .call::<_, ()>(winid)?)
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
        winid: usize,
        row: usize,
        col: usize,
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_set_cursor")?
            .call::<_, ()>((winid, [row, col]))?)
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
        winid: usize,
        name: &str,
        value: V,
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_set_option")?
            .call::<_, ()>((winid, name, value))?)
    }
}
