use mlua::{Function, Lua, Result, Table};

pub struct Nvim<'a>(pub Table<'a>);

impl<'a> Nvim<'a> {
    pub fn new(lua: &'a Lua) -> Result<Nvim<'a>> {
        Ok(Nvim(
            lua.globals()
                .get::<&str, Table>("vim")?
                .get::<&str, Table>("api")?,
        ))
    }
}

impl<'a> Nvim<'a> {
    /// Binding to `nvim_buf_add_highlight`.
    ///
    /// Returns the `ns_id` that was passed in if different from zero. When
    /// `ns_id = 0` a new namespace is created and the allocated id is
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `bufnr`      Buffer handle, or 0 for current buffer
    /// * `ns_id`      Namespace to use, or -1 for ungrouped highlight
    /// * `hl_group`   Name of the highlight group to use
    /// * `line`       Line to highlight (zero-indexed)
    /// * `col_start`  Start of column range to highlight (byte-indexed)
    /// * `col_end`    End of column range to highlight, or -1 to  highlight to end of line (byte-indexed)
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

    /// Binding to `nvim_buf_clear_namespace`.
    ///
    /// Clears namespaced objects (highlights, extmarks, virtual text) from a
    /// region. To clear the namespace in the entire buffer specify
    /// `line_start = 0` and `line_end = -1`.
    ///
    /// # Arguments
    ///
    /// * `bufnr`       Buffer handle, or 0 for current buffer
    /// * `ns_id`       Namespace to clear, or -1 to clear all namespaces
    /// * `line_start`  Start of range of lines to clear (zero-indexed)
    /// * `line_end`    End of range of lines to clear (zero-indexed and exclusive)
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

    /// Binding to `nvim_buf_set_extmark`.
    ///
    /// Creates or updates an extmark. To create a new extmark pass `ns_id =
    /// 0`. To move an extmark pass its `ns_id`. Returns the namespace id of
    /// the created/updates extmark.
    ///
    /// # Arguments
    ///
    /// * `bufnr`  Buffer handle, or 0 for current buffer
    /// * `ns_id`  Namespace id
    /// * `row`    Row where to place the extmark (0-indexed)
    /// * `col`    Column where to place the extmark (0-indexed)
    /// * `opts`   Optional parameters. See `:h nvim_buf_set_extmark` for  details
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

    /// Binding to `nvim_buf_set_lines`.
    ///
    /// Sets (replaces) a line-range in the buffer. Out-of-bounds indices are
    /// clamped to the nearest valid value, unless `strict_indexing` is set.
    ///
    /// # Arguments
    ///
    /// * `bufnr`            Buffer handle, or 0 for current buffer
    /// * `start`            First line index (zero-indexed)
    /// * `end`              Last line index (zero-indexed and exclusive)
    /// * `strict_indexing`  Whether out-of-bounds should be an error
    /// * `replacement`      Slice of lines to use as replacement
    pub fn buf_set_lines(
        &self,
        bufnr: usize,
        start: usize,
        end: isize,
        strict_indexing: bool,
        replacement: &[String],
    ) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_buf_set_lines")?
            .call::<_, ()>((
                bufnr,
                start,
                end,
                strict_indexing,
                replacement,
            ))?)
    }

    /// Binding to `nvim_buf_set_text`.
    ///
    /// Sets (replaces) a range in the buffer. Recommended over
    /// `Nvim::buf_set_lines` when modifying parts of a single line. To insert
    /// text at a given index set `start` and `end` to the same index.
    ///
    /// # Arguments
    ///
    /// * `bufnr`        Buffer handle, or 0 for current buffer
    /// * `start_row`    First line index (zero-indexed)
    /// * `start_col`    First column index (byte-indexed)
    /// * `end_row`      Last line index (zero-indexed and exclusive)
    /// * `end_col`      Last column index (byte-indexed and exclusive)
    /// * `replacement`  Slice of lines to use as replacement
    pub fn buf_set_text(
        &self,
        bufnr: usize,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
        replacement: &[&str],
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
                replacement,
            ))?)
    }

    /// Binding to `nvim_create_buf`.
    ///
    /// Creates a new, empty, unnamed buffer. Returns the new buffer handle, or
    /// 0 on error.
    ///
    /// # Arguments
    ///
    /// * `listed`   Whether to set `buflisted`
    /// * `scratch`  Whether the new buffer is a "throwaway" (`:h scratch-buffer`) buffer used for temporary work.
    pub fn create_buf(&self, listed: bool, scratch: bool) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_create_buf")?
            .call::<_, usize>((listed, scratch))?)
    }

    /// Binding to `nvim_create_namespace`.
    ///
    /// Creates a new namespace or gets an existing one. Namespaces can be
    /// named or anonymous. If `name` matches an existing namespace the
    /// associated id is returned. If `name` is an empty string a new,
    /// anonymous namespace is created.
    ///
    /// # Arguments
    ///
    /// * `name`   Namespace name or empty string
    pub fn create_namespace(&self, name: &str) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_create_namespace")?
            .call::<_, usize>(name)?)
    }

    /// Binding to `nvim_exec`.
    ///
    /// Executes a block of Vimscript. Returns the output if `output` is true,
    /// else an empty string.
    ///
    /// # Arguments
    ///
    /// * `src`     Vimscript code
    /// * `output`  Whether to Capture and return all output.
    pub fn exec(&self, src: &str, output: bool) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_exec")?
            .call::<_, ()>((src, output))?)
    }

    /// Binding to `nvim_get_current_line`
    ///
    /// Returns the current line.
    pub fn get_current_line(&self) -> Result<String> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_get_current_line")?
            .call::<_, String>(())?)
    }

    /// Binding to `nvim_open_win`.
    ///
    /// Opens a new window. Used to open floating and external windows.
    ///
    /// # Arguments
    ///
    /// * `bufnr`   Buffer to display, or 0 for current buffer
    /// * `enter`   Whether to enter the newly created window, making in the current window
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

    /// Binding to `nvim_get_cursor`
    ///
    /// Returns the (1,0)-indexed cursor position as a tuple.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window
    pub fn win_get_cursor(&self, winid: usize) -> Result<(usize, usize)> {
        let position = self
            .0
            .get::<&str, Function>("nvim_win_get_cursor")?
            .call::<_, Table>(winid)?;

        Ok((position.get(1)?, position.get(2)?))
    }

    /// Binding to `nvim_win_hide`.
    ///
    /// Closes the window and hides the buffer it contains.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window
    pub fn win_hide(&self, winid: usize) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_hide")?
            .call::<_, ()>(winid)?)
    }

    /// Binding to `nvim_set_cursor`
    ///
    /// Sets the (1,0)-indexed cursor position in the window.
    ///
    /// # Arguments
    ///
    /// * `winid`  Window handle, or 0 for current window
    /// * `pos`    `&[row, col]` slice representing the new cursor position
    pub fn win_set_cursor(&self, winid: usize, pos: &[usize]) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_set_cursor")?
            .call::<_, ()>((winid, pos))?)
    }
}
