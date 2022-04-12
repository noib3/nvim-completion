use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidSource {
    Lipsum,
    Lsp,
}
