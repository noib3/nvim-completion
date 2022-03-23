use std::ops::Range;

#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// The text to display in the details window as a vector of strings.
    pub details: Option<Vec<String>>,

    /// TODO: docs
    pub format: String,

    /// TODO: docs
    pub hl_ranges: Vec<(Range<usize>, &'static str)>,

    /// TODO: docs
    pub matched_bytes: u32,

    /// TODO: docs
    pub source: &'static str,

    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,
}

// impl CompletionItem {
//     pub fn new(
//         source: &'static str,
//         text: String,
//         details: Option<String>,
//         matched_bytes: Vec<usize>,
//     ) -> Self {
//         let details = details
//             .map(|lines| lines.lines().map(|line| line.to_string()).collect());

//         let hl_ranges =
//             vec![(1..matched_bytes as usize + 1, "CompleetMenuMatchingChars")];

//         CompletionItem {
//             details,
//             format: format!(" {}", text),
//             hl_ranges,
//             text,
//             source,
//         }
//     }
// }
