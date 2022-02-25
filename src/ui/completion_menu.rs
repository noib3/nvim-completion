use mlua::{Lua, Result};
use std::{cmp, fmt};

use crate::completion::CompletionItem;
use crate::Nvim;

pub struct CompletionMenu {
    /// TODO: docs
    bufnr: usize,

    /// TODO: docs
    winid: Option<usize>,

    /// TODO: docs
    pub selected_index: Option<usize>,
}

impl CompletionMenu {
    /// TODO: docs
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(CompletionMenu {
            bufnr: nvim.create_buf(false, true)?,
            winid: None,
            selected_index: None,
        })
    }
}

impl CompletionMenu {
    pub fn hide(&mut self, nvim: &Nvim) -> Result<()> {
        if let Some(winid) = &self.winid {
            nvim.win_hide(*winid)?;
            self.winid = None;
        }
        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        self.winid.is_some()
    }

    pub fn select_completion(
        &mut self,
        nvim: &Nvim,
        new_selected_index: Option<usize>,
    ) -> Result<()> {
        match self.selected_index {
            Some(old) => nvim.buf_clear_namespace(
                self.bufnr,
                -1,
                old,
                (old + 1).try_into().unwrap(),
            )?,
            None => {},
        };

        match new_selected_index {
            Some(new) => {
                nvim.buf_add_highlight(self.bufnr, -1, "Visual", new, 0, -1)?;
            },
            None => {},
        };

        self.selected_index = new_selected_index;

        Ok(())
    }

    pub fn show_completions(
        &mut self,
        nvim: &Nvim,
        lua: &Lua,
        completion_items: &[CompletionItem],
    ) -> Result<()> {
        let lines = completion_items
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        nvim.buf_set_lines(self.bufnr, 0, -1, false, &lines)?;

        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = cmp::min(lines.len(), 7);

        let config = lua.create_table_with_capacity(0, 8)?;
        config.set("relative", "cursor")?;
        config.set("width", width)?;
        config.set("height", height)?;
        config.set("row", 1)?;
        config.set("col", 0)?;
        config.set("focusable", false)?;
        config.set("style", "minimal")?;
        config.set("noautocmd", true)?;

        self.winid = Some(nvim.open_win(self.bufnr, false, config)?);

        Ok(())
    }
}

impl fmt::Display for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} ({}) ", self.text, self.matched_characters.len())
    }
}
