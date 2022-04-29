use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct LspConfig {
    pub enable: bool,

    #[serde(default)]
    pub highlight_completions: bool,
}
