use mlua::{prelude::LuaResult, Lua};
use neovim::Api;

use crate::settings::ui::border::Border;
use crate::ui::WindowPosition;

#[derive(Debug)]
pub struct CompletionDetails {
    /// The handle of the buffer used to show the completion details. It is
    /// set once on initialization and never changes.
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
    pub fn is_visible(&self) -> bool { self.winid.is_some() }

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
        border: &Border,
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

        if border.enable {
            opts.set("border", border.style.to_lua(lua)?)?;
        }

        let winid = api.open_win(self.bufnr, false, opts)?;
        api.win_set_option(
            winid,
            "winhl",
            "FloatBorder:CompleetDetailsBorder,Normal:CompleetDetails,Search:\
             None",
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
        maybe_lines: Option<&Vec<String>>,
        border: &Border,
        menu_width: u32,
        menu_winid: u32,
        menu_border: &Border,
        force_redraw: bool,
    ) -> LuaResult<()> {
        if maybe_lines.is_none() {
            self.close(api)?;
            return Ok(());
        }

        let lines = maybe_lines.expect("Already checked `None` variant");
        let maybe_position = super::get_position(
            api,
            lines,
            border,
            menu_winid,
            menu_width,
            menu_border,
        )?;

        match (self.is_visible(), maybe_position) {
            // The window is already visible and we have a new position. We
            // should just shift the window, but unfortunately because of a bug
            // (https://github.com/neovim/neovim/issues/17853) this doesn't
            // always work, sometimes we need to close and reopen it.
            (true, Some(position)) => {
                if force_redraw {
                    self.close(api)?;
                    self.spawn(lua, api, menu_winid, &position, border)?
                } else {
                    self.shift(lua, api, menu_winid, &position)?;
                }
                self.fill(api, lines)?;
            },

            // The window wasn't open but now we have a new position. We create
            // a new one and fill the buffer.
            (false, Some(position)) => {
                self.spawn(lua, api, menu_winid, &position, border)?;
                self.fill(api, lines)?;
            },

            // The window was open but there's nothign to display anymore. We
            // just close it.
            (true, None) => {
                self.close(api)?;
            },

            // The window wasn't open and there's still nothing to
            // display. This is a no-op.
            (false, None) => {},
        }

        Ok(())
    }
}
