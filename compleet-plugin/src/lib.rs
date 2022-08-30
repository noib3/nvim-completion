use compleet_core as core;
use compleet_lipsum as lipsum;
use compleet_lsp as lsp;
use nvim_oxi::{self as nvim, Dictionary};

#[nvim::module]
fn compleet() -> nvim::Result<Dictionary> {
    core::register_source(lsp::CompleetLsp);

    #[cfg(debug_assertions)]
    core::register_source(lipsum::CompleetLipsum);

    Ok(core::build_api())
}
