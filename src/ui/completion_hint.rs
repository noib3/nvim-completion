use mlua::{Lua, Result};

use crate::Nvim;

pub struct CompletionHint {
    /// The Neovim namespace id associated to the completion hint.
    ns_id: usize,

    /// TODO
    pub hinted_index: Option<usize>,
}

impl CompletionHint {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(CompletionHint {
            ns_id: nvim.create_namespace("compleet_completion_hint")?,
            hinted_index: None,
        })
    }
}

impl CompletionHint {
    pub fn erase(&mut self, nvim: &Nvim) -> Result<()> {
        // nvim.buf_clear_namespace(0, self.ns_id, 0, -1)?;
        nvim.buf_clear_namespace(
            0,
            (self.ns_id).try_into().unwrap_or(-1), // TODO: this is bad
            0,
            -1,
        )?;
        self.hinted_index = None;
        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        self.hinted_index.is_some()
    }

    pub fn set(
        &mut self,
        lua: &Lua,
        nvim: &Nvim,
        hinted_index: usize,
        col: usize,
        hint: &str,
    ) -> Result<()> {
        let opts = lua.create_table_with_capacity(0, 3)?;
        opts.set("id", 1)?;
        opts.set::<&str, &[&[&str]]>("virt_text", &[&[hint, "Comment"]])?;
        opts.set("virt_text_pos", "overlay")?;

        let row = nvim.win_get_cursor(0)?.0 - 1;
        nvim.buf_set_extmark(0, self.ns_id, row, col, opts)?;
        self.hinted_index = Some(hinted_index);

        Ok(())
    }
}
