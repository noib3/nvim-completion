use nvim_oxi::{self as nvim, api::Buffer, opts::BufAttachOpts, print};

use crate::Client;

/// Called the first time the user enters a new buffer.
pub(super) fn on_buf_enter(client: &Client, buf: Buffer) -> nvim::Result<()> {
    print!("Just entered {buf}");

    // if client.should_attach(&buf) {
    //     let opts = BufAttachOpts::builder().on_bytes(|args| Ok(true)).build();
    //     buf.attach(false, &opts)?;
    // }

    Ok(())
}
