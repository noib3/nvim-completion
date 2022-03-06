use mlua::Result;

use crate::Nvim;

pub struct DetailsPane {
    /// The handle of the buffer used to show details for the currently
    /// selected completion items.
    _bufnr: usize,

    /// The handle of the floating window used to show the detail infos, or
    /// `None` if the details pane is not currently visible.
    winid: Option<usize>,
}

impl DetailsPane {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(DetailsPane {
            _bufnr: nvim.create_buf(false, true)?,
            winid: None,
        })
    }

    pub fn hide(&mut self, nvim: &Nvim) -> Result<()> {
        if let Some(winid) = &self.winid {
            nvim.win_hide(*winid)?;
            self.winid = None;
        }
        Ok(())
    }

    fn _is_visible(&self) -> bool {
        self.winid.is_some()
    }
}
