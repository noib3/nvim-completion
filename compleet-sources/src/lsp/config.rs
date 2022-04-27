use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LspConfig {
    pub enable: bool,
    pub highlight_completions: bool,
}

impl Default for LspConfig {
    fn default() -> Self {
        LspConfig { enable: false, highlight_completions: false }
    }
}
