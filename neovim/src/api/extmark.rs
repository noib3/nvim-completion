use mlua::{Function, Result, Table};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_buf_add_highlight`.
    ///
    /// Returns the `ns_id` that was passed in if different from zero. When
    /// `ns_id = 0` a new namespace is created and the allocated id is
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `bufnr`      Buffer handle, or 0 for current buffer.
    /// * `ns_id`      Namespace to use, or -1 for ungrouped highlight.
    /// * `hl_group`   Name of the highlight group to use.
    /// * `line`       Line to highlight (zero-indexed).
    /// * `col_start`  Start of column range to highlight (byte-indexed).
    /// * `col_end`    End of column range to highlight, or -1 to  highlight to end of line (byte-indexed).
    pub fn buf_add_highlight(
        &self,
        bufnr: usize,
        ns_id: isize,
        hl_group: &str,
        line: usize,
        col_start: usize,
        col_end: isize,
    ) -> Result<isize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_add_highlight")?
            .call::<_, isize>((
                bufnr, ns_id, hl_group, line, col_start, col_end,
            ))?)
    }

    /// Binding to `vim.api.nvim_buf_clear_namespace`.
    ///
    /// Clears namespaced objects (highlights, extmarks, virtual text) from a
    /// region. To clear the namespace in the entire buffer specify
    /// `line_start = 0` and `line_end = -1`.
    ///
    /// # Arguments
    ///
    /// * `bufnr`       Buffer handle, or 0 for current buffer.
    /// * `ns_id`       Namespace to clear.
    /// * `line_start`  Start of range of lines to clear (zero-indexed).
    /// * `line_end`    End of range of lines to clear (zero-indexed and exclusive).
    pub fn buf_clear_namespace(
        &self,
        bufnr: usize,
        ns_id: isize,
        line_start: usize,
        line_end: isize,
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_clear_namespace")?
            .call::<_, ()>((bufnr, ns_id, line_start, line_end))?)
    }

    /// Binding to `vim.api.nvim_buf_set_extmark`.
    ///
    /// Creates or updates an extmark. To create a new extmark pass `ns_id =
    /// 0`. To move an extmark pass its `ns_id`. Returns the namespace id of
    /// the created/updates extmark.
    ///
    /// # Arguments
    ///
    /// * `bufnr`  Buffer handle, or 0 for current buffer.
    /// * `ns_id`  Namespace id.
    /// * `row`    Row where to place the extmark (0-indexed).
    /// * `col`    Column where to place the extmark (0-indexed).
    /// * `opts`   Optional parameters. See `:h nvim_buf_set_extmark` for  details.
    pub fn buf_set_extmark(
        &self,
        bufnr: usize,
        ns_id: usize,
        row: usize,
        col: usize,
        opts: Table,
    ) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_set_extmark")?
            .call::<_, usize>((bufnr, ns_id, row, col, opts))?)
    }

    /// Binding to `vim.api.nvim_create_namespace`.
    ///
    /// Creates a new namespace or gets an existing one. Namespaces can be
    /// named or anonymous. If `name` matches an existing namespace the
    /// associated id is returned. If `name` is an empty string a new,
    /// anonymous namespace is created.
    ///
    /// # Arguments
    ///
    /// * `name`   Namespace name or empty string.
    pub fn create_namespace(&self, name: &str) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_create_namespace")?
            .call::<_, usize>(name)?)
    }
}
