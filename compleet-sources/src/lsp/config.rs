use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LspConfig {
    pub enable: bool,

    #[serde(default)]
    pub highlight_completions: bool,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self { enable: false, highlight_completions: false }
    }
}

impl From<LspConfig> for super::Lsp {
    fn from(config: LspConfig) -> Self {
        Self { config, ..Default::default() }
    }
}
