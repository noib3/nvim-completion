use nvim::api::{self, opts::*, types::AutocmdCallbackArgs, Buffer};
use nvim_oxi as nvim;

use crate::{Client, Result};

/// TODO: docs
pub(crate) fn attach_to_buffer(
    client: &Client,
    augroup_id: u32,
    buf: Buffer,
) -> Result<()> {
    let on_cursor_moved_i =
        client.to_nvim_fn(|client, args: AutocmdCallbackArgs| {
            super::on_cursor_moved_i(client, args.buffer)?;
            Ok(false)
        });

    let on_insert_leave =
        client.to_nvim_fn(|client, args: AutocmdCallbackArgs| {
            super::on_insert_leave(client, args.buffer)?;
            Ok(false)
        });

    let on_vim_resized =
        client.to_nvim_fn(|client, _: AutocmdCallbackArgs| {
            super::on_vim_resized(client)?;
            Ok(false)
        });

    let mut builder = CreateAutocmdOpts::builder();
    builder.group(augroup_id).buffer(buf);

    api::create_autocmd(
        ["CursorMovedI"],
        &builder.clone().callback(on_cursor_moved_i).build(),
    )?;

    api::create_autocmd(
        ["InsertLeave"],
        &builder.clone().callback(on_insert_leave).build(),
    )?;

    api::create_autocmd(
        ["VimResized"],
        &builder.clone().callback(on_vim_resized).build(),
    )?;

    Ok(())
}
