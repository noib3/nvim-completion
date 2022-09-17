use nvim::api::Buffer;
use nvim_oxi as nvim;

use crate::Client;

pub(super) fn on_cursor_moved_i(
    client: &Client,
    buf: Buffer,
) -> nvim::Result<()> {
    client.stop_sending();

    Ok(())
}
