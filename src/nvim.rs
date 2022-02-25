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

    /// Binding to `nvim_buf_set_lines`.
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
    pub fn buf_set_text(
        &self,
        bufnr: usize,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
        replacement: &[String],
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
    pub fn create_buf(&self, listed: bool, scratch: bool) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_create_buf")?
            .call::<_, usize>((listed, scratch))?)
    }

    /// Binding to `nvim_exec`.
    pub fn exec(&self, src: &str, output: bool) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_exec")?
            .call::<_, ()>((src, output))?)
    }

    /// Binding to `nvim_get_current_buf`
    pub fn get_current_buf(&self) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_get_current_buf")?
            .call::<_, usize>(())?)
    }

    /// Binding to `nvim_get_current_line`
    pub fn get_current_line(&self) -> Result<String> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_get_current_line")?
            .call::<_, String>(())?)
    }

    /// Binding to `nvim_get_current_win`
    pub fn get_current_win(&self) -> Result<usize> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_get_current_win")?
            .call::<_, usize>(())?)
    }

    /// Binding to `nvim_open_win`.
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
    pub fn win_get_cursor(&self, handle: usize) -> Result<(usize, usize)> {
        let position = self
            .0
            .get::<&str, Function>("nvim_win_get_cursor")?
            .call::<_, Table>(handle)?;

        Ok((position.get(1)?, position.get(2)?))
    }

    /// Binding to `nvim_win_hide`.
    pub fn win_hide(&self, handle: usize) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_hide")?
            .call::<_, ()>(handle)?)
    }

    /// Binding to `nvim_set_cursor`
    pub fn win_set_cursor(&self, handle: usize, pos: Table) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_win_set_cursor")?
            .call::<_, ()>((handle, pos))?)
    }
}
