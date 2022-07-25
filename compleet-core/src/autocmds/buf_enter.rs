use nvim_oxi::{api::Buffer, opts::BufAttachOpts};

use crate::on_bytes;
use crate::Client;

/// Called the first time the user enters a new buffer.
pub(super) fn on_buf_enter(client: &Client, buf: Buffer) -> crate::Result<()> {
    if client.should_attach(&buf)? {
        let on_bytes = client.create_fn(on_bytes::on_bytes);
        let opts = BufAttachOpts::builder().on_bytes(on_bytes).build();
        buf.attach(false, &opts)?;
    }

    Ok(())
}
