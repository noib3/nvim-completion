use mlua::{Function, Result, Table};

use crate::Nvim;

pub fn nvim_create_buf(
    nvim: &Nvim,
    listed: bool,
    scratch: bool,
) -> Result<usize> {
    Ok(nvim
        .get::<&str, Table>("api")?
        .get::<&str, Function>("nvim_create_buf")?
        .call::<_, usize>((listed, scratch))?)
}
