use std::ops::Range;

#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// TODO: docs
    pub details: Option<Vec<String>>,

    // TODO: rename
    /// TODO: docs
    pub line: String,

    /// TODO: refactor
    pub matched_prefix_len: usize,

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
        matched_prefix_len: usize,
    ) -> Self {
        CompletionItem {
            details: details.map(|lines| {
                lines.lines().map(|line| line.into()).collect::<Vec<_>>()
            }),
            line: format!(" {}", text),
            matched_prefix_len,
            matched_byte_ranges: vec![(1..matched_prefix_len + 1)],
            text,
        }
    }
}
