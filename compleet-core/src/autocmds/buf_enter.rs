use nvim_oxi::api::Buffer;

use crate::Client;

/// Called the first time the user enters a new buffer.
pub(super) fn on_buf_enter(client: &Client, buf: Buffer) -> crate::Result<()> {
    if buf.get_option("modifiable")? {
        client.query_attach(buf);
    }

    Ok(())
}
