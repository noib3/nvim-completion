use nvim_oxi::{
    api,
    opts::CreateCommandOpts,
    types::{CommandArgs, CommandNArgs},
};

use crate::{Client, Result};

pub(crate) fn setup(client: &Client) -> Result<()> {
    let stats =
        client.to_nvim_fn(|client, _args| super::completion_stats(client));

    let start = client.to_nvim_fn(|client, args: CommandArgs| {
        super::completion_start(client, args.bang, args.fargs)
    });

    let stop = client.to_nvim_fn(|client, args: CommandArgs| {
        super::completion_stop(client, args.bang, args.fargs)
    });

    let opts = CreateCommandOpts::builder()
        .bang(true)
        .nargs(CommandNArgs::Any)
        .build();

    api::create_user_command("CompletionStats", stats, None)?;
    api::create_user_command("CompletionStart", start, Some(&opts))?;
    api::create_user_command("CompletionStop", stop, Some(&opts))?;

    Ok(())
}
