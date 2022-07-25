use compleet_core as compleet;
use compleet_lipsum as lipsum;
use nvim_oxi::{self as nvim, Dictionary};

#[nvim::module]
fn compleet() -> nvim::Result<Dictionary> {
    let client = compleet::Client::new();

    client.register_source(lipsum::CompleetLipsum);

    Ok(client.build_api())
}
