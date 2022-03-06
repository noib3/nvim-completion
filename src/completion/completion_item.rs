pub struct CompletionItem {
    /// The text that will be inserted into the buffer if a completion is
    /// selected.
    pub text: String,

    /// A vector of indices representing bytes of the `text` field that are
    /// matched by the current completion prefix.
    pub matched_bytes: Vec<usize>,
}

impl CompletionItem {
    pub fn new(text: String, matched_prefix: &str) -> Self {
        CompletionItem {
            text,
            matched_bytes: (0..matched_prefix.len()).collect(),
        }
    }
}
