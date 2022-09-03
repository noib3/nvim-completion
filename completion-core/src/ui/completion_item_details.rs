use nvim::api::{self, Buffer, Window};
use nvim_oxi as nvim;

use super::ui_config::DetailsConfig;

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
}
