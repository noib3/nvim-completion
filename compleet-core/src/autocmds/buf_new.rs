use nvim_oxi::{
    self as nvim,
    api::{self, Buffer},
    opts::CreateAutocmdOpts,
    types::AutocmdCallbackArgs,
};

use crate::Client;

pub(crate) fn on_buf_new(client: &Client, buf: Buffer) -> nvim::Result<()> {
    let buf_enter = client.create_fn(|client, args: AutocmdCallbackArgs| {
        super::on_buf_enter(client, args.buffer).map(|_| false)
    });

    let opts = CreateAutocmdOpts::builder()
        .buffer(buf)
        .callback(buf_enter)
        .once(true)
        .build();

    api::create_autocmd(["BufEnter"], &opts)?;

    Ok(())
}
