use nvim_oxi as nvim;

use crate::completions::CompletionBundle;
use crate::Client;

/// Function called every time a bunch of completion results computed by the
/// thread pool (potentially coming from different sources) are sent to the
/// main thread to be displayed.
pub(crate) fn on_completions_arrival(
    client: &Client,
    bundles: Vec<CompletionBundle>,
) -> nvim::Result<()> {
    // First we filter out bundles coming from old revisions.
    let mut iter = bundles
        .into_iter()
        .filter(|(_, req, _, _)| client.is_last_rev(&req.rev));

    let (source_name, req, completions, sorted) = match iter.next() {
        Some(b) => b,
        None => return Ok(()),
    };

    if source_name != "lipsum" {
        return Ok(());
    }

    nvim::print!(
        "arrived {} completion{} from {} in {:?}ms",
        completions.len(),
        if completions.len() != 1 { "s" } else { "" },
        source_name,
        req.start.elapsed().as_millis(),
    );

    let ciao = sorted.iter().map(|idx| &completions[*idx]).collect::<Vec<_>>();

    if ciao.is_empty() {
        return Ok(());
    }

    client.ui().display(&*ciao, &mut req.nvim_buf(), &req.ctx.line)?;

    nvim::print!(
        "displayed {} completion{} from {} in {:?}ms",
        completions.len(),
        if completions.len() != 1 { "s" } else { "" },
        source_name,
        req.start.elapsed().as_millis(),
    );

    nvim::print!("-------");

    Ok(())
}
