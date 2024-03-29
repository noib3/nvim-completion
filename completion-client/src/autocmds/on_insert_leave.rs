use nvim::api::Buffer;
use nvim_oxi as nvim;

use crate::Client;

pub(super) fn on_insert_leave(
    client: &Client,
    mut buf: Buffer,
) -> nvim::Result<()> {
    client.stop_sending();
    client.ui_mut().hide_all(&mut buf)
}
