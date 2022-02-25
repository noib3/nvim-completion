use super::CompletionItem;

pub struct CompletionState {
    /// TODO: docs
    pub current_line: String,

    /// TODO
    pub bytes_before_cursor: usize,

    /// TODO: docs
    pub matched_prefix: String,

    /// TODO: docs
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
