use mlua::{prelude::LuaResult, Lua};
use neovim::Api;

use crate::ui::WindowPosition;

#[derive(Debug)]
pub struct CompletionDetails {
    /// The handle of the buffer used to show the completion details. It is set
    /// once on initialization and never changes.
    bufnr: u32,

    /// The handle of the floating window used to show the completion details,
    /// or `None` if the details window is not currently visible.
    pub winid: Option<u32>,
}

impl CompletionDetails {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(CompletionDetails {
            bufnr: api.create_buf(false, true)?,
            winid: None,
        })
    }
}

impl CompletionDetails {
    /// Closes the details window.
    pub fn close(&mut self, api: &Api) -> LuaResult<()> {
        if let Some(winid) = self.winid {
            api.win_hide(winid)?;
            self.winid = None;
        }
        Ok(())
    }

    /// TODO: docs
    pub fn is_visible(&self) -> bool {
        self.winid.is_some()
    }

    /// TODO: docs
    pub fn fill(&mut self, api: &Api, lines: &[String]) -> LuaResult<()> {
        api.buf_set_lines(self.bufnr, 0, -1, false, lines)
    }

    /// TODO: docs
    pub fn shift(
        &mut self,
        lua: &Lua,
        api: &Api,
        menu_winid: u32,
        position: &WindowPosition,
    ) -> LuaResult<()> {
        let winid = self
            .winid
            .expect("The details window is visible so it has a window id.");

        let opts = lua.create_table_with_capacity(0, 7)?;
        opts.set("relative", "win")?;
        opts.set("win", menu_winid)?;
        opts.set("width", position.width)?;
        opts.set("height", position.height)?;
        opts.set("row", position.row)?;
        opts.set("col", position.col)?;

        api.win_set_config(winid, opts)?;

        Ok(())
    }

    /// TODO: docs
    pub fn spawn(
        &mut self,
        lua: &Lua,
        api: &Api,
        menu_winid: u32,
        position: &WindowPosition,
    ) -> LuaResult<()> {
        let opts = lua.create_table_with_capacity(0, 10)?;
        opts.set("relative", "win")?;
        opts.set("win", menu_winid)?;
        opts.set("width", position.width)?;
        opts.set("height", position.height)?;
        opts.set("row", position.row)?;
        opts.set("col", position.col)?;
        opts.set("focusable", false)?;
        opts.set("style", "minimal")?;
        opts.set("noautocmd", true)?;

        let winid = api.open_win(self.bufnr, false, opts)?;
        api.win_set_option(
            winid,
            "winhl",
            "Normal:CompleetDetails,Search:None",
        )?;
        api.win_set_option(winid, "scrolloff", 0)?;

        self.winid = Some(winid);

        Ok(())
    }
}
