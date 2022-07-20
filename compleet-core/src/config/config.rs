use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(default)]
    ui: super::UiConfig,

    #[serde(default)]
    completion: super::CompletionConfig,
}

impl Config {
    /// Whether completion hints are enabled.
    pub const fn hints_enabled(&self) -> bool {
        self.ui.hint.enable
    }
}
