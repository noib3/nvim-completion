use serde::{Deserialize, Serialize};

use super::lipsum;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcesSettings {
    #[serde(default)]
    pub lipsum: lipsum::LipsumSourceSettings,
}

impl Default for SourcesSettings {
    fn default() -> Self {
        SourcesSettings {
            lipsum: lipsum::LipsumSourceSettings::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSettings {
    pub enable: bool,
}
