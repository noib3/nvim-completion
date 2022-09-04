use nvim_oxi as nvim;

use crate::completions::CompletionBundle;
use crate::{Client, Result};

/// Function called every time a bunch of completion results computed by the
/// thread pool (potentially coming from different sources) are sent to the
/// main thread to be displayed.
pub(crate) fn on_completions_arrival(
    client: &Client,
    bundles: Vec<CompletionBundle>,
) -> Result<()> {
    // NOTE: all UI-related Neovim API calls cannot be executed directly at
    // this time or they may cause segfaults. Instead they have to be scheduled
    // to be executed on the next tick of the event loop.

    let client = client.clone();

    nvim::schedule(move |_| {
        // First we filter out bundles coming from old revisions.
        let iter = bundles
            .into_iter()
            .filter(|(_, req, _)| client.is_last_rev(&req.rev));

        for (source_name, req, completions) in iter {
            let ui = &mut *client.ui();

            nvim::print!(
                "arrived {} completion{} from {} in {:?}ms",
                completions.len(),
                if completions.len() != 1 { "s" } else { "" },
                source_name,
                req.start.elapsed().as_millis(),
            );

            ui.hint.show(
                &mut req.nvim_buf(),
                &(*req).ctx.cursor,
                &completions[0],
            )?;

            if !ui.menu.is_open() && completions.len() > 1 {
                ui.menu.open(&completions, &req.start)?;
            } else {
                // ui.menu.insert(&[(completions, 0)])?;
            }

            nvim::print!(
                "displayed {} completion{} from {} in {:?}ms",
                completions.len(),
                if completions.len() != 1 { "s" } else { "" },
                source_name,
                req.start.elapsed().as_millis(),
            );
        }

        Ok(())
    });

    Ok(())
}
