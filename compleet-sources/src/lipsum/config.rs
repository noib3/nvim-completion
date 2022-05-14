use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LipsumConfig {
    pub enable: bool,
}

impl Default for LipsumConfig {
    fn default() -> Self {
        Self { enable: false }
    }
}

impl From<LipsumConfig> for super::Lipsum {
    fn from(config: LipsumConfig) -> Self {
        Self { _config: config, ..Default::default() }
    }
}
