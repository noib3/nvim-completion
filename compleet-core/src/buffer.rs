use std::path::{Path, PathBuf};

use nvim_oxi::api::Buffer as NvimBuffer;
use tokio::sync::mpsc::UnboundedSender;

use crate::threads::MainMessage;

#[derive(Debug)]
pub struct Buffer {
    /// TODO: docs
    buf: NvimBuffer,

    /// TODO: docs
    file_path: PathBuf,

    /// TODO: docs
    cb_sender: UnboundedSender<MainMessage>,
}

// Public API.
impl Buffer {
    /// TODO: docs
    #[inline(always)]
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }
}

// Private API.
impl Buffer {
    /// TODO: docs
    #[inline]
    pub(crate) fn new(
        buf: NvimBuffer,
        cb_sender: UnboundedSender<MainMessage>,
    ) -> crate::Result<Self> {
        let file_path = buf.get_name()?;
        Ok(Self { buf, file_path, cb_sender })
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn nvim_buf(&self) -> NvimBuffer {
        // An [`NvimBuffer`] is just a newtype around an `i32` so cloning is
        // cheaper that returning a reference to it.
        self.buf.clone()
    }
}
