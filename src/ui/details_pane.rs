use mlua::Result;

use crate::Nvim;

pub struct DetailsPane {
    /// TODO: docs
    _bufnr: usize,

    /// TODO: docs
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
}
