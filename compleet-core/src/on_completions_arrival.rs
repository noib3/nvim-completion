use crate::completion_bundle::CompletionBundle;
use crate::messages::echoerr;
use crate::{Client, Result};

/// Function called every time a bunch of completion results computed by the
/// thread pool (potentially coming from different sources) are sent to the
/// main thread to be displayed.
pub(crate) fn on_completions_arrival(
    client: &Client,
    bundles: Vec<CompletionBundle>,
) -> Result<()> {
    // First we filter out bundles coming from old revisions and errors.
    let iter = bundles.into_iter().filter_map(|(name, req, res)| {
        if !client.is_last_rev(req.rev_id()) {
            return None;
        }

        match res {
            Ok(completions) => Some((name, req, completions)),

            Err(err) => {
                echoerr!("{:?}", err);
                None
            },
        }
    });

    for (source_name, req, completions) in iter {
        nvim_oxi::print!(
            "got {} completion{} from {} in {:?}ms",
            completions.len(),
            if completions.len() != 1 { "s" } else { "" },
            source_name,
            req.start.elapsed().as_millis(),
        );
    }

    Ok(())
}
