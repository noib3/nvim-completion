use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ValidSource {
    Lipsum,
    Lsp,
}
