use mlua::{Lua, Result};

use crate::Nvim;

pub struct CompletionHint {
    // TODO: docs
    ns_id: usize,

    // TODO: docs
    is_visible: bool,
}

impl CompletionHint {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(CompletionHint {
            ns_id: nvim.create_namespace("compleet_completion_hint")?,
            is_visible: false,
        })
    }
}

impl CompletionHint {
    pub fn erase(&mut self, nvim: &Nvim) -> Result<()> {
        if !self.is_visible {
            // nvim.buf_clear_namespace(0, self.ns_id, 0, -1)?;
            nvim.buf_clear_namespace(
                0,
                (self.ns_id).try_into().unwrap(), // TODO: this is bad
                0,
                -1,
            )?;
            self.is_visible = false;
        }
        Ok(())
    }

    pub fn set(
        &mut self,
        lua: &Lua,
        nvim: &Nvim,
        row: usize,
        col: usize,
        hint: &str,
    ) -> Result<()> {
        let opts = lua.create_table()?;
        opts.set("id", 1)?;
        opts.set::<&str, &[&[&str]]>("virt_text", &[&[hint, "Comment"]])?;
        opts.set("virt_text_pos", "overlay")?;

        nvim.buf_set_extmark(0, self.ns_id, row, col, opts)?;
        self.is_visible = true;
        Ok(())
    }
}
