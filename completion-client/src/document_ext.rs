use completion_types::{ClientSender, Document};
use nvim_oxi::{self as nvim, api::Buffer};

pub(crate) trait DocumentExt: Sized {
    fn new(buffer: Buffer, ui_sender: ClientSender) -> nvim::Result<Self>;
}

impl DocumentExt for Document {
    /// Has to be called on the Neovim thread.
    #[inline(always)]
    fn new(buffer: Buffer, ui_sender: ClientSender) -> nvim::Result<Self> {
        let path = buffer.get_name()?;
        Ok(Self { buffer, path, client_sender: ui_sender })
    }
}
