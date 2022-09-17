use nvim_oxi as nvim;

use crate::Client;

pub(super) fn on_vim_resized(client: &Client) -> nvim::Result<()> {
    // let ui = &mut *client.ui();

    // ui.update_columns()?;
    // ui.update_rows()?;

    Ok(())
}
