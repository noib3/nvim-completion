use mlua::Result;

use super::{Buffer, FloatingWindow};

use crate::Nvim;

pub struct DetailsPane {
    /// TODO: docs
    _buffer: Buffer,

    /// TODO: docs
    window: Option<FloatingWindow>,
}

impl DetailsPane {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(DetailsPane {
            _buffer: Buffer::new(nvim, false, true)?,
            window: None,
        })
    }

    pub fn hide(&mut self, nvim: &Nvim) -> Result<()> {
        if let Some(window) = &self.window {
            window.hide(nvim)?;
            self.window = None;
        }
        Ok(())
    }
}
