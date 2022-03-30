use std::ops::Range;

pub type Completions = Vec<CompletionItem>;

#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// The text to display in the details window as a vector of strings.
    pub details: Option<Vec<String>>,

    /// The formatted completion item as shown inside the completion menu.
    pub format: String,

    /// A vector or `(range, hl_group)` tuples, where each byte in the `range`
    /// is highlighted with the `hl_group` highlight group.
    pub hl_ranges: Vec<(Range<usize>, &'static str)>,

    /// The number of bytes before the current cursor position that are
    /// matched by the completion item.
    pub matched_bytes: u32,

    /// The name of the source this completion comes from.
    pub source: &'static str,

    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,
}
