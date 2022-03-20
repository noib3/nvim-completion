use serde::{Deserialize, Serialize};

use super::{completion, sources, ui};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default)]
    pub ui: ui::UiSettings,

    #[serde(default)]
    pub completion: completion::CompletionSettings,

    #[serde(default)]
    pub sources: sources::SourcesSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            ui: ui::UiSettings::default(),
            completion: completion::CompletionSettings::default(),
            sources: sources::SourcesSettings::default(),
        }
    }
}
