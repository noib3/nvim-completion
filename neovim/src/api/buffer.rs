use mlua::{FromLua, Function, Result, Table};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_buf_attach`.
    ///
    /// Activates buffer-update events on a channel, or as Lua callbacks.
    ///
    /// # Arguments
    ///
    /// * `bufnr`        Buffer handle, or 0 for current buffer.
    /// * `send_buffer`  Whether the initial notification should contain the whole buffer.
    /// * `opts`         Optional parameters. See `:h nvim_buf_attach` for  details.
    pub fn buf_attach(
        &self,
        bufnr: usize,
        send_buffer: bool,
        opts: Table,
    ) -> Result<bool> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_attach")?
            .call::<_, bool>((bufnr, send_buffer, opts))?)
    }

    /// Binding to `vim.api.nvim_buf_call`.
    ///
    /// Calls a function with `bufnr` as the temporary current buffer.
    ///
    /// # Arguments
    ///
    /// * `bufnr`   Buffer handle, or 0 for current buffer.
    /// * `fun`     Function to call inside the buffer.
    pub fn buf_call(&self, bufnr: usize, fun: Function) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_call")?
            .call::<_, ()>((bufnr, fun))?)
    }

    /// Binding to `vim.api.nvim_buf_get_lines`.
    ///
    /// Gets a line range from the buffer. Indexing is zero-based,
    /// end-exclusive. Negative indices are interpreted as length + 1 + index:
    /// -1 refers to the index past the end. So to get the last element use
    /// `start = -2` and `end = -1`.
    ///
    /// # Arguments
    ///
    /// * `bufnr`   Buffer handle, or 0 for current buffer.
    /// * `start`   First line index.
    /// * `end`     First line index.
    /// * `strict_indexing`   Whether out-of-bounds should be an error.
    pub fn buf_get_lines(
        &self,
        bufnr: usize,
        start: usize,
        end: isize,
        strict_indexing: bool,
    ) -> Result<Vec<String>> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_get_lines")?
            .call::<_, Vec<String>>((bufnr, start, end, strict_indexing))?)
    }

    /// Binding to `vim.api.nvim_buf_get_lines`.
    ///
    /// Gets a line range from the buffer. Indexing is zero-based,
    /// end-exclusive. Negative indices are interpreted as length + 1 + index:
    /// -1 refers to the index past the end. So to get the last element use
    /// `start = -2` and `end = -1`.
    ///
    /// # Arguments
    ///
    /// * `bufnr`   Buffer handle, or 0 for current buffer.
    /// * `start`   First line index.
    /// * `end`     First line index.
    /// * `strict_indexing`   Whether out-of-bounds should be an error.
    pub fn buf_get_option<V: FromLua<'a>>(
        &self,
        bufnr: usize,
        name: &str,
    ) -> Result<V> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_get_option")?
            .call::<_, V>((bufnr, name))?)
    }

    /// Binding to `vim.api.nvim_buf_set_lines`.
    ///
    /// Sets (replaces) a line-range in the buffer. Out-of-bounds indices are
    /// clamped to the nearest valid value, unless `strict_indexing` is set.
    ///
    /// # Arguments
    ///
    /// * `bufnr`            Buffer handle, or 0 for current buffer.
    /// * `start`            First line index (zero-indexed).
    /// * `end`              Last line index (zero-indexed and exclusive).
    /// * `strict_indexing`  Whether out-of-bounds should be an error.
    /// * `replacement`      Slice of lines to use as replacement.
    pub fn buf_set_lines<L: AsRef<str>>(
        &self,
        bufnr: usize,
        start: usize,
        end: isize,
        strict_indexing: bool,
        replacement: &[L],
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_set_lines")?
            .call::<_, ()>((
                bufnr,
                start,
                end,
                strict_indexing,
                replacement
                    .iter()
                    .map(|l| l.as_ref())
                    .collect::<Vec<&str>>(),
            ))?)
    }

    /// Binding to `vim.api.nvim_buf_set_text`.
    ///
    /// Sets (replaces) a range in the buffer. Recommended over
    /// `Api::buf_set_lines` when modifying parts of a single line. To insert
    /// text at a given index set `start` and `end` to the same index.
    ///
    /// # Arguments
    ///
    /// * `bufnr`        Buffer handle, or 0 for current buffer.
    /// * `start_row`    First line index (zero-indexed).
    /// * `start_col`    First column index (byte-indexed).
    /// * `end_row`      Last line index (zero-indexed and exclusive).
    /// * `end_col`      Last column index (byte-indexed and exclusive).
    /// * `replacement`  Slice of lines to use as replacement.
    pub fn buf_set_text<L: AsRef<str>>(
        &self,
        bufnr: usize,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
        replacement: &[L],
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_set_text")?
            .call::<_, ()>((
                bufnr,
                start_row,
                start_col,
                end_row,
                end_col,
                replacement
                    .iter()
                    .map(|l| l.as_ref())
                    .collect::<Vec<&str>>(),
            ))?)
    }
}
