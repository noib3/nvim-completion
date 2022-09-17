use nvim_oxi::api;
use nvim_oxi::opts::{CreateAugroupOpts, CreateAutocmdOpts};
use nvim_oxi::types::AutocmdCallbackArgs;

use crate::{Client, Result};

const AUGROUP_NAME: &str = "nvim-completion";

pub(crate) fn setup(client: &Client) -> Result<u32> {
    let augroup_id = {
        let opts = CreateAugroupOpts::builder().clear(true).build();
        api::create_augroup(AUGROUP_NAME, Some(&opts))?
    };

    let on_buf_new = client.to_nvim_fn(|client, args: AutocmdCallbackArgs| {
        super::on_buf_new(client, args.buffer)?;
        Ok(false)
    });

    let opts = CreateAutocmdOpts::builder()
        .group(augroup_id)
        .callback(on_buf_new)
        .build();

    api::create_autocmd(["BufNew"], &opts)?;

    Ok(augroup_id)
}
