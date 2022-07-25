use nvim_oxi::{
    opts::{OnBytesArgs, ShouldDetach},
    print,
};

use crate::Client;

pub(crate) fn on_bytes(
    client: &Client,
    args: OnBytesArgs,
) -> crate::Result<ShouldDetach> {
    print!("{args:?}");
    Ok(false)
}
