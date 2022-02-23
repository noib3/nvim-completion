use mlua::{Function, Result, Table};

use crate::Nvim;

pub fn nvim_buf_add_highlight(
    nvim: &Nvim,
    bufnr: usize,
    start: usize,
    end: isize,
    strict_indexing: bool,
    replacement: &[String],
) -> Result<()> {
    Ok(nvim
        .get::<&str, Table>("api")?
        .get::<&str, Function>("nvim_buf_set_lines")?
        .call::<_, ()>((bufnr, start, end, strict_indexing, replacement))?)
}
