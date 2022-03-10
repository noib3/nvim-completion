use std::ops::Range;

pub struct CompletionItem {
    /// TODO: docs
    pub details: Option<Vec<String>>,

    /// A vector of ranges representing indices of bytes of the `text` field
    /// that are matched by the current completion prefix.
    pub matched_byte_ranges: Vec<Range<usize>>,

    /// The text that will be inserted into the buffer if a completion is
    /// selected.
    pub text: String,
}

impl CompletionItem {
    pub fn new(
        text: String,
        details: Option<String>,
        matched_prefix: &str,
    ) -> Self {
        CompletionItem {
            details: details.map(|lines| {
                lines.lines().map(|line| line.into()).collect::<Vec<_>>()
            }),
            matched_byte_ranges: vec![(0..matched_prefix.len())],
            text,
        }
    }
}
