use std::time::Instant;

use nvim::api;
use nvim::opts::{OnBytesArgs, ShouldDetach};
use nvim_oxi as nvim;

use crate::completions::{LineContext, RevId};
use crate::{Client, CompletionContext, Result};

pub(crate) fn on_bytes(
    client: &Client,
    (
        _,
        buf,
        changedtick,
        start_row,
        start_col,
        _byte_offset,
        rows_deleted,
        _cols_deleted,
        bytes_deleted,
        rows_added,
        _cols_added,
        bytes_added,
    ): OnBytesArgs,
) -> Result<ShouldDetach> {
    let start = Instant::now();

    // We only care about insert mode events.
    if !api::get_mode()?.mode.is_insert() {
        return Ok(false);
    }

    // If we've added or deleted a line we return early. If we've stayed on the
    // same line but we've deleted characters we only continue if the
    // `completion.while_deleting` option is set.
    if rows_added != 0
        || rows_deleted != 0
        || (bytes_deleted != 0 && !false/* TODO */)
    {
        return Ok(false);
    }

    let line_ctx = {
        let row = start_row;

        let col = start_col + if bytes_deleted != 0 { 0 } else { bytes_added };

        let line = buf
            .get_lines(row, row + 1, true)?
            .next()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        LineContext::new(row, col, line)
    };

    let ctx = CompletionContext::new(line_ctx);

    let rev = RevId::new(buf, changedtick);

    client.query_completions(start, ctx, rev);

    Ok(false)
}
