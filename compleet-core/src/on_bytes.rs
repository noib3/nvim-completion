use std::time::Instant;

use nvim_oxi::opts::{OnBytesArgs, ShouldDetach};

use crate::completion_bundle::RevId;
use crate::cursor::Cursor;
use crate::{Client, CompletionContext};

pub(crate) fn on_bytes(
    client: &Client,
    args: OnBytesArgs,
) -> crate::Result<ShouldDetach> {
    let start = Instant::now();

    let buf = &args.1;
    let changedtick = args.2;

    let rev_id = RevId::new(buf.clone(), changedtick);

    client.stop_sources();
    client.set_rev_id(rev_id.clone());

    let ctx = CompletionContext::new(Cursor::try_from(&args)?);

    client.query_completions(buf, ctx, start, rev_id);

    Ok(false)
}
