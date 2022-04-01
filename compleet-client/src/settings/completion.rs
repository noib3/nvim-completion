use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionSettings {
    pub while_deleting: bool,
}

impl Default for CompletionSettings {
    fn default() -> Self {
        CompletionSettings {
            while_deleting: false,
        }
    }
}
