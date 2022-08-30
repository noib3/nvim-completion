use nvim_completion_core as core;
use nvim_completion_lipsum as lipsum;
use nvim_completion_lsp as lsp;
use nvim_oxi::{self as nvim, Dictionary};

#[nvim::module]
fn nvim_completion() -> nvim::Result<Dictionary> {
    core::register_source(lsp::Lsp);

    #[cfg(debug_assertions)]
    core::register_source(lipsum::Lipsum);

    Ok(core::build_api())
}
