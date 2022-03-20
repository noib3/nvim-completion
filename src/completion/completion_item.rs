use std::ops::Range;

#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// The text to display in the details window as a vector of strings.
    pub details: Option<Vec<String>>,

    /// TODO: docs
    pub line: String,

    /// TODO: docs
    pub hl_ranges: Vec<(Range<usize>, &'static str)>,

    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,
}

impl CompletionItem {
    pub fn new(
        text: String,
        details: Option<String>,
        matched_bytes: u32,
    ) -> Self {
        let details = details
            .map(|lines| lines.lines().map(|line| line.to_string()).collect());

        let hl_ranges =
            vec![(1..matched_bytes as usize + 1, "CompleetMenuMatchingChars")];

        CompletionItem {
            details,
            line: format!(" {}", text),
            hl_ranges,
            text,
        }
    }
}
