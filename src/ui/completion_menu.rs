use core::ops::Range;
use mlua::{Lua, Result};
use std::cmp;

use neovim::{Api, Neovim};

use crate::completion::CompletionItem;

pub enum MenuPosition {
    /// TODO: docs
    Below(usize),
}

pub struct CompletionMenu {
    /// The handle of the buffer used to show the completion items. It is set
    /// once on initialization and never changes.
    bufnr: usize,

    /// TODO: docs
    visible_range: Option<Range<usize>>,

    /// A namespace id used to handle the highlighting of characters matching
    /// the current completion prefix. It is set once on initialization and
    /// never changed.
    matched_chars_nsid: usize,

    /// The maximum height of the completion menu, or `None` if no max height
    /// has been set by the user.
    pub max_height: Option<usize>,

    /// TODO: docs
    pub position: Option<MenuPosition>,

    /// The index of the currently selected completion item, or `None` if no
    /// completion is selected. If `Some` it ranges from 0 to
    /// `completion_items.len() - 1`.
    pub selected_completion: Option<usize>,

    /// A namespace id used to handle the highlighting of the currently
    /// selected completion item. It is set once on initialization and never
    /// changed.
    selected_completion_nsid: usize,

    /// The handle of the floating window used to show the completion items, or
    /// `None` if the completion menu is not currently visible.
    pub winid: Option<usize>,
}

impl CompletionMenu {
    pub fn new(api: &Api) -> Result<Self> {
        Ok(CompletionMenu {
            bufnr: api.create_buf(false, true)?,
            visible_range: None,
            matched_chars_nsid: api
                .create_namespace("CompleetMatchedChars")?,
            max_height: None,
            position: None,
            selected_completion: None,
            selected_completion_nsid: api
                .create_namespace("CompleetSelectedItem")?,
            winid: None,
        })
    }
}

impl CompletionMenu {
    /// Clears the highlighting of a row of the completion menu from being
    /// marked as selected.
    fn clear_selected_completion(&self, api: &Api, row: usize) -> Result<()> {
        api.buf_clear_namespace(
            self.bufnr,
            self.selected_completion_nsid.try_into().unwrap_or(-1),
            row,
            (row + 1).try_into().unwrap_or(-1),
        )
    }

    /// TODO: docs
    fn create_floatwin(
        &self,
        lua: &Lua,
        api: &Api,
        width: usize,
        height: usize,
    ) -> Result<usize> {
        let config = lua.create_table_with_capacity(0, 8)?;
        config.set("relative", "cursor")?;
        config.set("width", width)?;
        config.set("height", height)?;
        config.set("row", 1)?;
        config.set("col", 0)?;
        config.set("focusable", false)?;
        config.set("style", "minimal")?;
        config.set("noautocmd", true)?;

        let winid = api.open_win(self.bufnr, false, config)?;
        api.win_set_option(winid, "winhl", "Normal:CompleetMenu")?;
        api.win_set_option(winid, "scrolloff", 0)?;
        Ok(winid)
    }

    /// TODO: docs
    pub fn hide(&mut self, api: &Api) -> Result<()> {
        if let Some(winid) = self.winid {
            api.win_hide(winid)?;
            self.winid = None;
        }
        // TODO: for now we reset the selected completion to `None` every time
        // the completion menu is hidden. We might want not to do this if we
        // can manage to differentiate a `move completion window` from a `close
        // completion window` commands.
        self.position = None;
        self.selected_completion = None;
        self.visible_range = None;
        Ok(())
    }

    /// TODO: docs
    pub fn is_item_selected(&self) -> bool {
        self.selected_completion.is_some()
    }

    /// TODO: docs
    pub fn is_visible(&self) -> bool {
        self.winid.is_some()
    }

    /// Adds highlighting to a row of the completion menu to mark it as
    /// selected.
    fn mark_completion_as_selected(
        &self,
        api: &Api,
        row: usize,
    ) -> Result<()> {
        api.buf_add_highlight(
            self.bufnr,
            self.selected_completion_nsid.try_into().unwrap_or(-1),
            "CompleetMenuSelected",
            row,
            0,
            -1,
        )?;
        Ok(())
    }

    /// TODO: docs
    pub fn select_completion(
        &mut self,
        lua: &Lua,
        api: &Api,
        new_selected_completion: Option<usize>,
    ) -> Result<()> {
        // Remove the highlighting from the previous selected completion.
        if let Some(old) = self.selected_completion {
            self.clear_selected_completion(api, old)?;
        }

        // Set the highlighting for the new selected completion.
        if let Some(new) = new_selected_completion {
            self.mark_completion_as_selected(api, new)?;
        }

        // Check if we need to scroll the buffer to keep the selected
        // completion visible.
        if let Some(range) = &mut self.visible_range {
            if is_scroll_needed(range, new_selected_completion) {
                put_row_at_top(lua, api, self.bufnr, range.start)?;
            }
        }

        self.selected_completion = new_selected_completion;

        Ok(())
    }

    /// TODO: docs
    // fn set_lines<L: AsRef<str>>(
    fn set_lines(&self, api: &Api, lines: &[String]) -> Result<()> {
        api.buf_set_lines(self.bufnr, 0, -1, false, lines)
    }

    /// TODO: docs
    pub fn show_completions(
        &mut self,
        lua: &Lua,
        api: &Api,
        completions: &[CompletionItem],
    ) -> Result<()> {
        let max_width = completions
            .iter()
            // TODO: Should use len of grapheme clusters, not bytes.
            .map(|item| item.text.len())
            .max()
            .unwrap_or(0);

        let height = match self.max_height {
            None => completions.len(),
            Some(height) => cmp::min(height, completions.len()),
        };

        let lines = completions
            .iter()
            .map(|item| item.format(max_width))
            .collect::<Vec<String>>();

        let width = max_width + 2;

        self.set_lines(api, &lines)?;
        self.winid = Some(self.create_floatwin(lua, api, width, height)?);
        self.position = Some(MenuPosition::Below(width));

        // We only track the visible range if we have some constraints on
        // `self.max_height` which we'll need to consider when selecting
        // completions.
        if self.max_height.is_some() {
            self.visible_range = Some(Range {
                start: 0,
                end: height,
            });
        }

        // TODO: make this into its own method.
        //
        // TODO: look into `:h nvim_set_decoration_provider` + `ephemeral`
        // option. What do they do? This seems to work fine w/o them but
        // nvim-cmp uses them.
        //
        let opts = lua.create_table_with_capacity(0, 4)?;
        opts.set("hl_group", "CompleetMenuMatchingChars")?;
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
                api.buf_set_extmark(
                    self.bufnr,
                    self.matched_chars_nsid,
                    row,
                    byte_range.start + 1,
                    _opts,
                )?;
            }
        }

        Ok(())
    }
}

/// TODO: docs
fn is_scroll_needed(
    range: &mut Range<usize>,
    new_selected_index: Option<usize>,
) -> bool {
    if let Some(index) = new_selected_index {
        if range.contains(&index) {
            return false;
        } else if index < range.start {
            range.end -= range.start - index;
            range.start = index;
        } else if index >= range.end {
            range.start += index + 1 - range.end;
            range.end = index + 1;
        }
        return true;
    }

    false
}

/// TODO: docs
fn put_row_at_top(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    row: usize,
) -> Result<()> {
    let fun = lua.create_function(move |lua, ()| {
        let _nvim = Neovim::new(lua)?;
        _nvim.api.command(&format!("normal! {}zt", row + 1))
    })?;

    api.buf_call(bufnr, fun)
}

impl CompletionItem {
    fn format(&self, padding: usize) -> String {
        format!(" {: <padding$} ", self.text)
    }
}
