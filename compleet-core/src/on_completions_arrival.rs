use crate::messages::echoerr;
use crate::{Client, CompletionBundle, Result};

/// Function called every time a bunch of completion results computed by the
/// thread pool (potentially coming from different sources) are sent to the
/// main thread to be displayed.
pub(crate) fn on_completions_arrival(
    client: &Client,
    bundles: Vec<CompletionBundle>,
) -> Result<()> {
    //
    // let reqq = &bundles.first().as_ref().unwrap().1;

    // First we filter out bundles coming from old revisions and errors.
    let iter = bundles.into_iter().filter_map(|(name, req, res)| {
        if !client.is_last_rev(&req.rev) {
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

    // let (req, iter) = match self::filter_old_and_errors(bundles) {
    //     (Some(req), iter) =>
    //     (None)
    // }

    for (source_name, req, completions) in iter {
        if let Some(comp) = completions.first() {
            client.ui().hint.show(
                &mut req.nvim_buf(),
                &(*req).ctx.cursor,
                comp,
            )?;
        }

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

// fn filter_old_and_errors(
//     bundles: Vec<CompletionBundle>,
// ) -> (
//     Option<Arc<CompletionRequest>>,
//     impl Iterator<Item = (SourceId, Vec<CompletionItem>)>,
// ) {
//     todo!()
// }
