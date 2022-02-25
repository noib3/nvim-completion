use mlua::{Lua, Result};

use crate::Nvim;

pub struct FloatingWindow {
    /// TODO: docs
    handle: usize,
}

impl FloatingWindow {
    pub fn new(
        nvim: &Nvim,
        lua: &Lua, // TODO: don't do this, convert config from hashmap
        bufnr: usize,
        width: usize,
        height: usize,
    ) -> Result<Self> {
        let config = lua.create_table_with_capacity(0, 8)?;
        config.set("relative", "cursor")?;
        config.set("width", width)?;
        config.set("height", height)?;
        config.set("row", 1)?;
        config.set("col", 0)?;
        config.set("focusable", false)?;
        config.set("style", "minimal")?;
        config.set("noautocmd", true)?;

        Ok(FloatingWindow {
            handle: nvim.open_win(bufnr, false, config)?,
        })
    }

    pub fn hide(&self, nvim: &Nvim) -> Result<()> {
        Ok(nvim.win_hide(self.handle)?)
    }
}
