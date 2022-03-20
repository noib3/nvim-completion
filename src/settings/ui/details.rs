use serde::{Deserialize, Serialize};

use super::borders;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetailsSettings {
    #[serde(default)]
    borders: borders::BorderSettings,
}

impl Default for DetailsSettings {
    fn default() -> Self {
        DetailsSettings {
            borders: borders::BorderSettings::default(),
        }
    }
}
