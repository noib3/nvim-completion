use nvim_oxi::{
    api,
    opts::CreateCommandOpts,
    types::{CommandArgs, CommandNArgs},
};

use crate::{Client, Result};

pub(crate) fn setup(client: &Client) -> Result<()> {
    let stats =
        client.to_nvim_fn(|client, _args| super::compleet_stats(client));

    let start = client.to_nvim_fn(|client, args: CommandArgs| {
        super::compleet_start(client, args.bang, args.fargs)
    });

    let stop = client.to_nvim_fn(|client, args: CommandArgs| {
        super::compleet_stop(client, args.bang, args.fargs)
    });

    let opts = CreateCommandOpts::builder()
        .bang(true)
        .nargs(CommandNArgs::Any)
        .build();

    api::create_user_command("CompleetStats", stats, None)?;
    api::create_user_command("CompleetStart", start, Some(&opts))?;
    api::create_user_command("CompleetStop", stop, Some(&opts))?;

    Ok(())
}
