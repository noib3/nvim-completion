use mlua::Result;

use crate::Nvim;

pub struct Buffer {
    /// TODO: docs
    pub bufnr: usize,
}

impl Buffer {
    pub fn add_highlight(&self, nvim: &Nvim, line: usize) -> Result<()> {
        nvim.buf_add_highlight(self.bufnr, -1, "Visual", line, 0, -1)?;
        Ok(())
    }

    pub fn clear_namespace(&self, nvim: &Nvim, line: usize) -> Result<()> {
        nvim.buf_clear_namespace(
            self.bufnr,
            -1,
            line,
            (line + 1).try_into().unwrap(),
        )?;
        Ok(())
    }

    pub fn new(nvim: &Nvim, listed: bool, scratch: bool) -> Result<Self> {
        Ok(Buffer {
            bufnr: nvim.create_buf(listed, scratch)?,
        })
    }

    // TODO: make this generic over: Vec<String>, &[&str], &[String], Vec<&str>
    pub fn set_lines(&self, nvim: &Nvim, lines: &[String]) -> Result<()> {
        Ok(nvim.buf_set_lines(self.bufnr, 0, -1, false, lines)?)
    }
}
