use nvim_oxi::{
    self as nvim,
    api,
    opts::CreateCommandOpts,
    types::CommandArgs,
};

use crate::Client;

pub(crate) fn setup(client: &Client) -> nvim::Result<()> {
    let stats =
        client.create_fn(|client, _args| super::compleet_stats(client));

    let start = client.create_fn(|client, args: CommandArgs| {
        super::compleet_start(client, args.bang, args.fargs)
    });

    let stop = client.create_fn(|client, args: CommandArgs| {
        super::compleet_stop(client, args.bang, args.fargs)
    });

    let opts = CreateCommandOpts::builder().bang(true).build();

    api::create_user_command("CompleetStats", stats, None)?;
    api::create_user_command("CompleetStart", start, Some(&opts))?;
    api::create_user_command("CompleetStop", stop, Some(&opts))?;

    Ok(())
}
