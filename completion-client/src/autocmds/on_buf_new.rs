use nvim_oxi::api::{
    self,
    opts::CreateAutocmdOpts,
    types::AutocmdCallbackArgs,
    Buffer,
};

use crate::{Client, Result};

pub(super) fn on_buf_new(client: &Client, buf: Buffer) -> Result<()> {
    let buf_enter = client.to_nvim_fn(|client, args: AutocmdCallbackArgs| {
        super::on_buf_enter(client, args.buffer)?;
        Ok(false)
    });

    let opts = CreateAutocmdOpts::builder()
        .buffer(buf)
        .callback(buf_enter)
        .once(true)
        .build();

    api::create_autocmd(["BufEnter"], &opts)?;

    Ok(())
}
