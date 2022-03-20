use mlua::{prelude::LuaResult, Lua};
use neovim::Api;

use crate::completion::CompletionItem;
use crate::ui::WindowPosition;

#[derive(Debug)]
pub struct CompletionMenu {
    /// The handle of the buffer used to show the completion items. It is set
    /// once on initialization and never changes.
    bufnr: u32,

    /// A namespace id used to handle the highlighting of characters matching
    /// the current completion prefix. It is set once on initialization and
    /// never changed.
    mc_nsid: u32,

    /// The index of the currently selected completion item, or `None` if no
    /// completion is selected.
    pub selected_index: Option<usize>,

    /// TODO: docs
    pub width: Option<u32>,

    /// The handle of the floating window used to show the completion items, or
    /// `None` if the completion menu is not currently visible.
    pub winid: Option<u32>,
}

impl CompletionMenu {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(CompletionMenu {
            bufnr: api.create_buf(false, true)?,
            mc_nsid: api.create_namespace("compleet_matched_chars")?,
            selected_index: None,
            width: None,
            winid: None,
        })
    }
}

impl CompletionMenu {
    /// Closes the completion menu, while also resetting the selected
    /// completion and the window position to `None`.
    pub fn close(&mut self, api: &Api) -> LuaResult<()> {
        if let Some(winid) = self.winid {
            api.win_hide(winid)?;
            self.winid = None;
        }
        self.selected_index = None;
        self.width = None;
        Ok(())
    }

    /// Fills the completion buffer with the completion results.
    pub fn fill(
        &mut self,
        lua: &Lua,
        api: &Api,
        completions: &[CompletionItem],
    ) -> LuaResult<()> {
        let lines = completions
            .iter()
            .map(|c| c.line.as_ref())
            .collect::<Vec<&str>>();

        api.buf_set_lines(self.bufnr, 0, -1, false, &lines)?;

        // Highlight some characters of the completion item.
        let opts = lua.create_table_with_capacity(0, 4)?;
        for (row, completion) in completions.iter().enumerate() {
            for range in &completion.hl_ranges {
                // TODO: the id has to be unique not only for every line
                // but also for every range.
                opts.set("hl_group", range.1)?;
                opts.set("id", row + 1)?;
                opts.set("end_row", row)?;
                opts.set("end_col", range.0.end)?;
                api.buf_set_extmark(
                    self.bufnr,
                    self.mc_nsid,
                    row as u32,
                    range.0.start as u32,
                    opts.clone(),
                )?;
            }
        }

        Ok(())
    }

    /// Whether a completion item is currently selected.
    pub fn is_item_selected(&self) -> bool {
        self.selected_index.is_some()
    }

    /// Whether the completion menu is visible.
    pub fn is_visible(&self) -> bool {
        self.winid.is_some()
    }

    /// Moves the completion menu to a new position.
    pub fn shift(
        &mut self,
        lua: &Lua,
        api: &Api,
        position: &WindowPosition,
    ) -> LuaResult<()> {
        let winid = self
            .winid
            .expect("The completion menu is visible so it has a window id.");

        let opts = lua.create_table_with_capacity(0, 5)?;
        opts.set("relative", "cursor")?;
        opts.set("height", position.height)?;
        opts.set("width", position.width)?;
        opts.set("row", position.row)?;
        opts.set("col", position.col)?;

        api.win_set_config(winid, opts)?;

        self.width = Some(position.width);

        Ok(())
    }

    /// Spawns the completion menu at a specified position.
    pub fn spawn(
        &mut self,
        lua: &Lua,
        api: &Api,
        position: &WindowPosition,
    ) -> LuaResult<()> {
        let opts = lua.create_table_with_capacity(0, 8)?;
        opts.set("relative", "cursor")?;
        opts.set("height", position.height)?;
        opts.set("width", position.width)?;
        opts.set("row", position.row)?;
        opts.set("col", position.col)?;
        opts.set("focusable", false)?;
        opts.set("style", "minimal")?;
        opts.set("noautocmd", true)?;

        let winid = api.open_win(self.bufnr, false, opts)?;
        api.win_set_option(
            winid,
            "winhl",
            "CursorLine:CompleetMenuSelected,Normal:CompleetMenu,Search:None",
        )?;
        api.win_set_option(winid, "scrolloff", 0)?;

        self.width = Some(position.width);
        self.winid = Some(winid);

        Ok(())
    }

    /// Selects a new completion.
    pub fn select(
        &mut self,
        api: &Api,
        new_selected_index: Option<usize>,
    ) -> LuaResult<()> {
        let winid = self
            .winid
            .expect("The completion menu is visible so it has a window id");

        match new_selected_index {
            Some(index) => {
                api.win_set_cursor(winid, (index + 1).try_into().unwrap(), 0)?;
                if self.selected_index.is_none() {
                    api.win_set_option(winid, "cursorline", true)?;
                }
            },

            None => api.win_set_option(winid, "cursorline", false)?,
        }

        self.selected_index = new_selected_index;

        Ok(())
    }
}
