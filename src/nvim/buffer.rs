use mlua::Result;

use super::{nvim_buf_set_lines, nvim_create_buf};
use crate::Nvim;

pub struct Buffer(usize);

impl Buffer {
    pub fn new(nvim: &Nvim, listed: bool, scratch: bool) -> Result<Self> {
        Ok(Buffer(nvim_create_buf(nvim, listed, scratch)?))
    }

    // TODO: make this generic over: Vec<String>, &[&str], &[String], Vec<&str>
    pub fn set_lines(&self, nvim: &Nvim, lines: &[String]) -> Result<()> {
        Ok(nvim_buf_set_lines(nvim, self.0, 0, -1, false, lines)?)
    }
}
