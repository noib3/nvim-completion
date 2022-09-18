use std::path::{Path, PathBuf};

use nvim::api::Buffer;
use nvim_oxi as nvim;

use crate::CoreSender;

/// TODO: docs
#[derive(Debug)]
pub struct Document {
    /// TODO: docs
    // #[cfg_attr(any(feature = "ui", feature = "core"), visibility::make(pub))]
    pub buffer: Buffer,

    /// TODO: docs
    // #[cfg_attr(any(feature = "ui", feature = "core"), visibility::make(pub))]
    pub path: PathBuf,

    /// TODO: docs
    // #[cfg_attr(feature = "ui", visibility::make(pub))]
    pub client_sender: CoreSender,
}

impl Document {
    /// TODO: docs
    #[inline(always)]
    pub fn path(&self) -> &Path {
        &self.path
    }

    // /// Has to be called on the Neovim thread.
    // #[inline(always)]
    // fn new(buffer: Buffer, ui_sender: UiSender) -> nvim::Result<Self> {
    //     let path = buffer.get_name()?;
    //     Ok(Self { buffer, path, ui_sender })
    // }

    // #[cfg_attr(any(feature = "ui", feature = "core"), visibility::make(pub))]
    #[inline(always)]
    pub fn buffer(&self) -> Buffer {
        self.buffer.clone()
    }
}
