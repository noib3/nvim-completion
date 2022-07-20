use compleet_core as compleet;
use nvim_oxi::{self as nvim, Dictionary, Result};

#[nvim::module]
fn compleet() -> Result<Dictionary> {
    let client = compleet::Client::new();

    Ok(Dictionary::from_iter([("setup", client.setup())]))
}
