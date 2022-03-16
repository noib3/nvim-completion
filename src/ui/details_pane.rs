use mlua::{Lua, Result};
use neovim::Api;
use std::cmp;

use super::positioning::{self, Error};

#[derive(Debug)]
pub struct DetailsPane {
    /// The handle of the buffer used to show details for the currently
    /// selected completion items.
    bufnr: usize,

    /// The handle of the floating window used to show the detail infos, or
    /// `None` if the details pane is not currently visible.
    winid: Option<usize>,
}

impl DetailsPane {
    pub fn new(api: &Api) -> Result<Self> {
        Ok(DetailsPane {
            bufnr: api.create_buf(false, true)?,
            winid: None,
        })
    }
}

impl DetailsPane {
    /// TODO: docs
    pub fn hide(&mut self, api: &Api) -> Result<()> {
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
    pub fn show(
        &mut self,
        lua: &Lua,
        api: &Api,
        lines: &[String],
        completion_menu_winid: usize,
        completion_menu_dimensions: (usize, usize),
    ) -> Result<()> {
        self.hide(api)?;

        let max_width = lines
            .iter()
            // TODO: Should use len of grapheme clusters, not bytes.
            .map(|line| line.len())
            .max()
            .unwrap_or(0);

        let width = cmp::min(max_width, 79);
        let height = lines.len();

        api.buf_set_lines(self.bufnr, 0, -1, false, lines)?;
        self.winid = match positioning::details::create_floatwin(
            lua,
            api,
            self.bufnr,
            width,
            height,
            completion_menu_winid,
            completion_menu_dimensions,
        ) {
            Ok(winid) => Some(winid),

            Err(err) => match err {
                Error::Lua(e) => return Err(e),

                // We don't really care why it failed, we just return early.
                _ => return Ok(()),
            },
        };

        Ok(())
    }
}
