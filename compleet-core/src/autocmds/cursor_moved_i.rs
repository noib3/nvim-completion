use nvim_oxi as nvim;

use crate::Client;

pub(super) fn on_cursor_moved_i(client: &Client) -> nvim::Result<()> {
    client.stop_sources();
    Ok(())
}
