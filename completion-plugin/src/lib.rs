use completion_client as client;
use completion_lipsum as lipsum;
use completion_lsp as lsp;
use nvim_oxi::{self as nvim, Dictionary};

#[nvim::module]
fn nvim_completion() -> nvim::Result<Dictionary> {
    client::register_source(lsp::Lsp);

    // #[cfg(debug_assertions)]
    client::register_source(lipsum::Lipsum);

    Ok(client::build_api())
}
