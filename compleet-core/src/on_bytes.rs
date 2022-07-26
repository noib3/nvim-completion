use nvim_oxi::opts::{OnBytesArgs, ShouldDetach};

use crate::{edit::Edit, Client};

pub(crate) fn on_bytes(
    client: &Client,
    args: OnBytesArgs,
) -> crate::Result<ShouldDetach> {
    client.apply_edit(&args.1, Edit::try_from(&args)?)?;
    Ok(false)
}
