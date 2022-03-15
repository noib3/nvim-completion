use mlua::{Lua, Result};
use neovim::Api;

#[derive(Debug)]
pub struct CompletionHint {
    /// The Neovim namespace id associated to the completion hint.
    ns_id: usize,

    /// TODO
    pub hinted_index: Option<usize>,
}

impl CompletionHint {
    pub fn new(api: &Api) -> Result<Self> {
        Ok(CompletionHint {
            ns_id: api.create_namespace("compleet_completion_hint")?,
            hinted_index: None,
        })
    }
}

impl CompletionHint {
    pub fn erase(&mut self, api: &Api) -> Result<()> {
        // nvim.buf_clear_namespace(0, self.ns_id, 0, -1)?;
        api.buf_clear_namespace(
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
        api: &Api,
        hinted_index: usize,
        col: usize,
        hint: &str,
    ) -> Result<()> {
        let opts = lua.create_table_with_capacity(0, 3)?;
        opts.set("id", 1)?;
        opts.set::<&str, &[&[&str]]>("virt_text", &[&[hint, "CompleetHint"]])?;
        opts.set("virt_text_pos", "overlay")?;

        let row = api.win_get_cursor(0)?.0 - 1;
        api.buf_set_extmark(0, self.ns_id, row, col, opts)?;
        self.hinted_index = Some(hinted_index);

        Ok(())
    }
}
