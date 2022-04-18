use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LipsumConfig {
    pub enable: bool,
}

impl Default for LipsumConfig {
    fn default() -> Self {
        LipsumConfig { enable: false }
    }
}
