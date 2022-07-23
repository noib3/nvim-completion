use nvim_oxi as nvim;

use crate::Client;

pub(super) fn on_insert_leave(client: &Client) -> nvim::Result<()> {
    Ok(())
}
