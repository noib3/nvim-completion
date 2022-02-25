pub struct CompletionItem {
    /// TODO: docs
    pub text: String,

    /// TODO: docs
    pub matched_characters: Vec<usize>,
}

impl CompletionItem {
    pub fn new(text: String, matched_prefix: &str) -> Self {
        CompletionItem {
            text,

            // TODO: len() won't work w/ multi bytes chars. Use
            // graphemes/bytes?
            matched_characters: (0..matched_prefix.len()).collect(),
        }
    }
}
