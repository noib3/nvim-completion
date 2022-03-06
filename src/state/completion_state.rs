use crate::completion::CompletionItem;

pub struct CompletionState {
    /// The text in the line at the current cursor position.
    pub current_line: String,

    /// Number of bytes before (usually to be read as left-of, except for
    /// right-to-left languages).
    pub bytes_before_cursor: usize,

    /// The string we're using to find completion candidates.
    pub matched_prefix: String,

    /// The completion candidates computed by `completion::algo::complete`.
    pub completion_items: Vec<CompletionItem>,
}

impl CompletionState {
    pub fn new() -> Self {
        CompletionState {
            current_line: String::from(""),
            bytes_before_cursor: 0,
            matched_prefix: String::from(""),
            completion_items: Vec::new(),
        }
    }
}
