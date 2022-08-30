use nvim_oxi::{api, opts::CreateAutocmdOpts, types::AutocmdCallbackArgs};

use crate::{Client, Result};

pub(crate) fn setup(client: &Client, augroup_id: u32) -> Result<()> {
    let on_buf_new = client.to_nvim_fn(|client, args: AutocmdCallbackArgs| {
        super::on_buf_new(client, args.buffer)?;
        Ok(false)
    });

    let opts = CreateAutocmdOpts::builder()
        .group(augroup_id)
        .callback(on_buf_new)
        .build();

    api::create_autocmd(["BufNew"], &opts)?;

    Ok(())
}
