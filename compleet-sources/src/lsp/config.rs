use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LspConfig {
    pub enable: bool,
    pub test: String,
}

impl Default for LspConfig {
    fn default() -> Self {
        LspConfig { enable: false, test: "Default".into() }
    }
}
