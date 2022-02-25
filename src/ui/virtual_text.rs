use mlua::Result;

use crate::Nvim;

pub struct VirtualText {
    text: Option<String>,
}

impl VirtualText {
    pub fn new() -> Self {
        VirtualText { text: None }
    }

    pub fn erase(&mut self, _nvim: &Nvim) -> Result<()> {
        self.text = None;
        Ok(())
    }
}
