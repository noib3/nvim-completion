use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HintSettings {
    #[serde(default)]
    pub enable: bool,
}

impl Default for HintSettings {
    fn default() -> Self {
        HintSettings {
            enable: bool::default(),
        }
    }
}
