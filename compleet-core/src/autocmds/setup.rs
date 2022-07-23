use nvim_oxi::{
    self as nvim,
    api,
    opts::CreateAutocmdOpts,
    types::AutocmdCallbackArgs,
};

use crate::Client;

pub(crate) fn setup(client: &Client) -> nvim::Result<()> {
    let buf_new = client.create_fn(|client, args: AutocmdCallbackArgs| {
        super::on_buf_new(client, args.buffer).map(|_| false)
    });

    api::create_autocmd(
        ["BufNew"],
        &CreateAutocmdOpts::builder().callback(buf_new).build(),
    )?;

    Ok(())
}
