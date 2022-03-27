use serde::Deserialize;

use super::lipsum::Lipsum;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcesSettings {
    #[serde(default)]
    pub lipsum: Lipsum,
}

impl Default for SourcesSettings {
    fn default() -> Self {
        SourcesSettings {
            lipsum: Lipsum::default(),
        }
    }
}
