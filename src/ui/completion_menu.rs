use mlua::{Lua, Result};
use std::cmp;

use neovim::Api;

use super::positioning::{self, WindowPosition};
use crate::completion::CompletionItem;

#[derive(Debug)]
pub struct CompletionMenu {
    /// The handle of the buffer used to show the completion items. It is set
    /// once on initialization and never changes.
    pub bufnr: usize,

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

    /// The handle of the floating window used to show the completion items, or
    /// `None` if the completion menu is not currently visible.
    pub winid: Option<usize>,
}

impl CompletionMenu {
    pub fn new(api: &Api) -> Result<Self> {
        Ok(CompletionMenu {
            bufnr: api.create_buf(false, true)?,
            matched_chars_nsid: api
                .create_namespace("CompleetMatchedChars")?,
            position: None,
            selected_completion: None,
            winid: None,
        })
    }
}

impl CompletionMenu {
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

    /// TODO: docs
    pub fn select_completion(
        &mut self,
        api: &Api,
        new_selected_completion: Option<usize>,
    ) -> Result<()> {
        let winid = self
            .winid
            .expect("The completion menu is visible so it has a winid");

        match new_selected_completion {
            Some(index) => {
                if self.selected_completion.is_none() {
                    api.win_set_option(winid, "cursorline", true)?;
                }
                api.win_set_cursor(winid, index + 1, 0)?
            },

            None => api.win_set_option(winid, "cursorline", false)?,
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
    api.win_set_option(
        winid,
        "winhl",
        "CursorLine:CompleetMenuSelected,Normal:CompleetMenu,Search:None",
    )?;
    api.win_set_option(winid, "scrolloff", 0)?;

    Ok(winid)
}

pub fn fill_buffer(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    nsid: usize,
    completions: &[CompletionItem],
) -> Result<()> {
    let lines = completions
        .iter()
        .map(|item| item.format())
        .collect::<Vec<String>>();

    api.buf_set_lines(bufnr, 0, -1, false, &lines)?;

    // Highlight the matched characters of every completion result.
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
