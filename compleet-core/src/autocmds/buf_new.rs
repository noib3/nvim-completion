use nvim_oxi::{self as nvim, api::Buffer};

use crate::Client;

pub(crate) fn on_buf_new(client: &Client, buf: Buffer) -> nvim::Result<()> {
    todo!()
}
