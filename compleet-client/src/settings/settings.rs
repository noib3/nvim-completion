use serde::Deserialize;
use sources::sources::Sources;

use super::completion::CompletionSettings;
use super::ui::UiSettings;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default)]
    pub ui: UiSettings,

    #[serde(default)]
    pub completion: CompletionSettings,

    #[serde(default, with = "super::sources")]
    pub sources: Sources,
}
