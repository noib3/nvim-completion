use super::IdentifyCompletion;
use crate::{Client, Result};

pub(super) fn select_completion(
    client: &Client,
    which: IdentifyCompletion,
) -> Result<()> {
    let ui = &mut *client.ui_mut();

    match which {
        IdentifyCompletion::ByIndex(idx) => todo!(),
        IdentifyCompletion::FromSelected(offset) => todo!(),
    }

    Ok(())
}
