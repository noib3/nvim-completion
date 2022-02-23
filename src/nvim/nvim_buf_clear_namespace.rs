use mlua::{Function, Result, Table};

use crate::Nvim;

pub fn nvim_buf_clear_namespace(
    nvim: &Nvim,
    bufnr: usize,
    ns_id: isize,
    line_start: usize,
    line_end: isize,
) -> Result<()> {
    Ok(nvim
        .get::<&str, Table>("api")?
        .get::<&str, Function>("nvim_buf_clear_namespace")?
        .call::<_, ()>((bufnr, ns_id, line_start, line_end))?)
}
