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

    /// TODO: document
    selected_item_ns_id: usize,

    /// TODO: document
    matched_chars_ns_id: usize,
}

impl CompletionMenu {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(CompletionMenu {
            bufnr: nvim.create_buf(false, true)?,
            _max_height: None,
            selected_index: None,
            winid: None,
            selected_item_ns_id: nvim
                .create_namespace("CompleetSelectedItem")?,
            matched_chars_ns_id: nvim
                .create_namespace("CompleetMatchedChars")?,
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
                self.selected_item_ns_id.try_into().unwrap_or(-1),
                old,
                (old + 1).try_into().unwrap_or(-1), // TODO: this is bad
            )?,
            None => {},
        };

        match new_selected_index {
            Some(new) => {
                nvim.buf_add_highlight(
                    self.bufnr,
                    self.selected_item_ns_id.try_into().unwrap_or(-1),
                    "CompleetMenuSelected",
                    new,
                    0,
                    -1,
                )?;
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
        completions: &[CompletionItem],
    ) -> Result<()> {
        let max_width = completions
            .iter()
            .map(|item| item.text.len())
            .max()
            .unwrap_or(0);

        let lines = completions
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

        let winid = nvim.open_win(self.bufnr, false, config)?;
        nvim.win_set_option(winid, "winhl", "Normal:CompleetMenu")?;
        self.winid = Some(winid);

        let opts = lua.create_table_with_capacity(0, 4)?;
        opts.set("hl_group", "CompleetMenuMatchingChars")?;

        // TODO: look into `:h nvim_set_decoration_provider` + `ephemeral`
        // option. What do they do? This seems to work fine w/o them but
        // nvim-cmp uses them.
        for (row, completion) in completions.iter().enumerate() {
            for byte_range in &completion.matched_byte_ranges {
                // The `+1` to the byte range start and end is needed because
                // of the space prepended to every completion item by
                // `CompletionItem::format`.
                let _opts = opts.clone();
                // TODO: the id has to be unique not only for every line but
                // also for every range. Find a way to combine the two.
                _opts.set("id", row + 1)?;
                _opts.set("end_row", row)?;
                _opts.set("end_col", byte_range.end + 1)?;
                nvim.buf_set_extmark(
                    self.bufnr,
                    self.matched_chars_ns_id,
                    row,
                    byte_range.start + 1,
                    _opts,
                )?;
            }
        }

        Ok(())
    }
}

impl CompletionItem {
    fn format(&self, padding: usize) -> String {
        format!(" {: <padding$} ", self.text)
    }
}
