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
    winid: Option<u32>,
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
    pub fn _shift(
        &mut self,
        lua: &Lua,
        api: &Api,
        menu_winid: u32,
        position: &WindowPosition,
    ) -> LuaResult<()> {
        let winid = self
            .winid
            .expect("The details window is visible so it has a window id.");

        let opts = lua.create_table_with_capacity(0, 6)?;
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
    fn spawn(
        &mut self,
        lua: &Lua,
        api: &Api,
        menu_winid: u32,
        position: &WindowPosition,
    ) -> LuaResult<()> {
        let opts = lua.create_table_with_capacity(0, 9)?;
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

    /// TODO: docs
    pub fn update(
        &mut self,
        lua: &Lua,
        api: &Api,
        new_lines: Option<&Vec<String>>,
        menu_width: u32,
        menu_winid: u32,
    ) -> LuaResult<()> {
        match new_lines {
            // If the are new lines to fill the buffer with try to get a
            // position for the floating window.
            Some(lines) => {
                match (
                    self.is_visible(),
                    &super::get_position(api, lines, menu_winid, menu_width)?,
                ) {
                    (true, Some(position)) => {
                        // TODO: Understand why closing and reopening the
                        // details window works but setting a new config makes
                        // it lag by 1 column. I tried reproducing this by
                        // replicating the same command sequence manually but
                        // everything seems to work fine. What is going on
                        // here?
                        //
                        // This lags 1 column behind.
                        //
                        // self.shift(lua, api, menu_winid, position)?;

                        // So closing and reopening works but shifting
                        // doesn't?
                        self.close(api)?;
                        self.spawn(lua, api, menu_winid, position)?;

                        self.fill(api, lines)?;
                    },

                    (false, Some(position)) => {
                        self.spawn(lua, api, menu_winid, position)?;
                        self.fill(api, lines)?;
                    },

                    (true, None) => {
                        self.close(api)?;
                    },

                    (false, None) => {},
                }
            },

            // Otherwise close the window.
            None => self.close(api)?,
        }

        Ok(())
    }
}
