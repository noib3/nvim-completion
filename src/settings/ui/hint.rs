use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
