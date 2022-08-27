use nvim_oxi::{
    self as nvim,
    api,
    opts::CreateAutocmdOpts,
    types::AutocmdCallbackArgs,
};

use crate::Client;

pub(crate) fn setup(client: &Client, augroup_id: u32) -> nvim::Result<()> {
    let on_buf_new = client.create_fn(|client, args: AutocmdCallbackArgs| {
        super::on_buf_new(client, args.buffer)?;
        Ok::<_, nvim::Error>(false)
    });

    let opts = CreateAutocmdOpts::builder()
        .group(augroup_id)
        .callback(on_buf_new)
        .build();

    api::create_autocmd(["BufNew"], &opts)?;

    Ok(())
}
