use mlua::{Lua, Result};
use std::cmp;

use crate::completion::CompletionItem;
use crate::Nvim;

pub struct CompletionMenu {
    /// The handle of the buffer used to show the completion items.
    bufnr: usize,

    /// The maximum height of the completion menu, or `None` if no max height
    /// has been set by the user.
    _max_height: Option<usize>,

    /// The index of the currently selected completion item, or `None` if no
    /// completion is selected.
    pub selected_index: Option<usize>,

    /// The handle of the floating window used to show the completion items, or
    /// `None` if the completion menu is not currently visible.
    winid: Option<usize>,
}

impl CompletionMenu {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(CompletionMenu {
            bufnr: nvim.create_buf(false, true)?,
            _max_height: None,
            selected_index: None,
            winid: None,
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

    pub fn is_item_selected(&self) -> bool {
        self.selected_index.is_some()
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
                (old + 1).try_into().unwrap_or(-1), // TODO: this is bad
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
        let max_width = completion_items
            .iter()
            .map(|item| item.text.len())
            .max()
            .unwrap_or(0);

        let lines = completion_items
            .iter()
            .map(|item| item.format(max_width))
            .collect::<Vec<String>>();

        nvim.buf_set_lines(self.bufnr, 0, -1, false, &lines)?;

        let config = lua.create_table_with_capacity(0, 8)?;
        config.set("relative", "cursor")?;
        config.set("width", max_width + 2)?;
        config.set("height", cmp::min(lines.len(), 7))?;
        config.set("row", 1)?;
        config.set("col", 0)?;
        config.set("focusable", false)?;
        config.set("style", "minimal")?;
        config.set("noautocmd", true)?;

        self.winid = Some(nvim.open_win(self.bufnr, false, config)?);

        Ok(())
    }
}

impl CompletionItem {
    fn format(&self, padding: usize) -> String {
        format!(" {: <padding$} ", self.text)
    }
}
