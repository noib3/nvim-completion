use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CompletionConfig {
    /// Whether to show completion results right after a completion has been
    /// accepted. If `false`, after accepting a completion new results will
    /// only be shown after typing more characters.
    pub(super) after_inserting: bool,

    /// Whether to show completion results while deleting. If set to `false`
    /// completions will only be shown automatically when inserting characters.
    pub(super) while_deleting: bool,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self { after_inserting: false, while_deleting: false }
    }
}
