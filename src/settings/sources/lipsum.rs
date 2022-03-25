use serde::Deserialize;

use super::sources::SourceSettings;

#[derive(Debug, Deserialize)]
pub struct LipsumSettings {
    #[serde(flatten, default = "lipsum_default_source")]
    pub source: SourceSettings,
}

impl Default for LipsumSettings {
    fn default() -> Self {
        LipsumSettings {
            source: lipsum_default_source(),
        }
    }
}

fn lipsum_default_source() -> SourceSettings {
    SourceSettings { enable: false }
}
