use serde::{Deserialize, Serialize};

use super::sources::SourceSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct LipsumSourceSettings {
    #[serde(flatten, default = "lipsum_default_source")]
    pub source: SourceSettings,
}

impl Default for LipsumSourceSettings {
    fn default() -> Self {
        LipsumSourceSettings {
            source: lipsum_default_source(),
        }
    }
}

fn lipsum_default_source() -> SourceSettings {
    SourceSettings { enable: false }
}
