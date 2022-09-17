use nvim::api::{self, Buffer, Window};
use nvim_oxi as nvim;
use nvim_oxi::types::WindowBorder;
use serde::Deserialize;

use super::Border;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DetailsConfig {
    #[serde(default = "default_details_border")]
    border: Border,
}

fn default_details_border() -> Border {
    Border {
        enable: true,
        style: WindowBorder::from((
            None,
            None,
            None,
            (' ', "CompletionDetails"),
        )),
    }
}

impl Default for DetailsConfig {
    #[inline]
    fn default() -> Self {
        Self { border: default_details_border() }
    }
}

#[derive(Debug)]
pub(crate) struct CompletionItemDetails {
    config: DetailsConfig,

    /// TODO: docs
    buf: Buffer,

    win: Option<Window>,

    /// TODO: docs
    height: u16,

    /// TODO: docs
    width: u16,
}

impl Default for CompletionItemDetails {
    #[inline]
    fn default() -> Self {
        Self {
            config: DetailsConfig::default(),
            buf: 0.into(),
            win: None,
            height: 0,
            width: 0,
        }
    }
}

impl CompletionItemDetails {
    #[inline]
    pub(super) fn init(&mut self, config: DetailsConfig) -> nvim::Result<()> {
        self.config = config;
        self.buf = api::create_buf(false, true)?;

        Ok(())
    }

    /// Hides the completion details window if currently visible, does nothing
    /// otherwise.
    pub(super) fn hide(&mut self) -> nvim::Result<()> {
        if let Some(win) = self.win.take() {
            win.hide()?;
        }

        Ok(())
    }
}
