use core::ops::Range;
use mlua::{Lua, Result};
use std::cmp;

use neovim::{Api, Neovim};

use super::positioning::{self, WindowPosition};
use crate::completion::CompletionItem;

#[derive(Debug)]
pub struct CompletionMenu {
    /// The handle of the buffer used to show the completion items. It is set
    /// once on initialization and never changes.
    pub bufnr: usize,

    /// TODO: docs
    visible_range: Option<Range<usize>>,

    /// A namespace id used to handle the highlighting of characters matching
    /// the current completion prefix. It is set once on initialization and
    /// never changed.
    pub matched_chars_nsid: usize,

    // TODO: this should belong w/ winid. They are Some and None together.
    /// TODO: docs
    pub position: Option<WindowPosition>,

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
            position: None,
            selected_completion: None,
            selected_completion_nsid: api
                .create_namespace("CompleetSelectedItem")?,
            winid: None,
        })
    }
}

impl CompletionMenu {
    /// Clears the highlighting from a row of the completion menu to no longer
    /// mark it as selected.
    fn clear_selected_completion(&self, api: &Api, row: usize) -> Result<()> {
        api.buf_clear_namespace(
            self.bufnr,
            self.selected_completion_nsid.try_into().unwrap_or(-1),
            row,
            (row + 1).try_into().unwrap_or(-1),
        )
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
        self.selected_completion = None;
        self.visible_range = None;
        self.position = None;
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
        // Remove the highlighting from the currently selected completion.
        if let Some(old) = self.selected_completion {
            self.clear_selected_completion(api, old)?;
        }

        // Set the highlighting for the newly selected completion.
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

    // TODO: rename
    /// TODO: docs
    pub fn show_completions(
        &mut self,
        api: &Api,
        completions: &[CompletionItem],
        max_height: Option<usize>,
    ) -> Result<Option<WindowPosition>> {
        let max_width = completions
            .iter()
            // TODO: Use length of grapheme clusters, not bytes.
            .map(|item| item.text.len())
            .max()
            .expect("There's at least one completion");

        let width = max_width + 2;

        let height = match max_height {
            None => completions.len(),
            Some(height) => cmp::min(height, completions.len()),
        };

        // We only track the visible range if we have some constraints on
        // `max_height`, which we'll need to consider when scrolling through
        // completions.
        if max_height.is_some() {
            self.visible_range = Some(Range {
                start: 0,
                end: height,
            });
        }

        // Getting the completion position might fail if the current window is
        // not big enough (either vertically, horizontally, or both) to contain
        // it.
        let position = match positioning::menu::get_winpos(api, width, height)
        {
            Ok(position) => position,

            Err(err) => match err {
                positioning::Error::Lua(e) => return Err(e),
                _ => return Ok(None),
            },
        };

        // TODO: get rid of this.
        self.position = Some(position.clone());

        Ok(Some(position))
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
        let nvim = Neovim::new(lua)?;
        nvim.api.command(&format!("normal! {}zt", row + 1))
    })?;

    api.buf_call(bufnr, fun)
}

/// Creates the floating window for the completion menu, returning its window
/// id.
pub fn create_floatwin(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    position: &WindowPosition,
) -> Result<usize> {
    let opts = lua.create_table_with_capacity(0, 8)?;
    opts.set("relative", "cursor")?;
    opts.set("width", position.width)?;
    opts.set("height", position.height)?;
    opts.set("row", position.row)?;
    opts.set("col", position.col)?;
    opts.set("focusable", false)?;
    opts.set("style", "minimal")?;
    opts.set("noautocmd", true)?;

    let winid = api.open_win(bufnr, false, opts)?;
    api.win_set_option(winid, "winhl", "Normal:CompleetMenu")?;
    api.win_set_option(winid, "scrolloff", 0)?;

    Ok(winid)
}

pub fn fill_buffer(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    max_width: usize,
    nsid: usize,
    completions: &[CompletionItem],
) -> Result<()> {
    let lines = completions
        .iter()
        .map(|item| item.format(max_width))
        .collect::<Vec<String>>();

    api.buf_set_lines(bufnr, 0, -1, false, &lines)?;

    // Finally, highlight the matched characters of every completion result.
    let opts = lua.create_table_with_capacity(0, 4)?;
    opts.set("hl_group", "CompleetMenuMatchingChars")?;

    for (row, completion) in completions.iter().enumerate() {
        for byte_range in &completion.matched_byte_ranges {
            // The `+1` to the byte range start and end is needed
            // because of the space prepended to every completion item
            // by `CompletionItem::format`.
            let _opts = opts.clone();
            // TODO: the id has to be unique not only for every line
            // but also for every range. Find a way to combine the two.
            _opts.set("id", row + 1)?;
            _opts.set("end_row", row)?;
            _opts.set("end_col", byte_range.end + 1)?;
            api.buf_set_extmark(
                bufnr,
                nsid,
                row,
                byte_range.start + 1,
                _opts,
            )?;
        }
    }

    Ok(())
}
