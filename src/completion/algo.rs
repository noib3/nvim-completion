use super::{CompletionItem, Cursor};

/// This is the main function responsible for the autocompletion. Given the
/// current line and the cursor position, it generates a vector of completion
/// results.
pub fn complete(cursor: &Cursor) -> Vec<CompletionItem> {
    if cursor.matched_bytes == 0 {
        return Vec::new();
    }

    let entries = [
        ("foo", Some("A foo")),
        ("bar", Some("A bar")),
        ("baz", Some("A baz")),
        ("bam", None),
        ("bazooka", Some("A bazooka")),
        ("baroo", None),
        ("barometer", None),
        ("beard", None),
        ("bear", None),
        ("bamm", None),
        ("bamboozled", None),
        ("bambi", None),
    ];

    let matched_text = &cursor.line[((cursor.at_bytes - cursor.matched_bytes)
        as usize)
        ..(cursor.at_bytes as usize)];

    let completions = entries
        .iter()
        .filter(|&&entry| {
            entry.0.starts_with(matched_text) && entry.0 != matched_text
        })
        .map(|entry| {
            CompletionItem::new(
                entry.0.to_string(),
                entry.1.map(|d| d.into()),
                cursor.matched_bytes,
            )
        })
        .collect::<Vec<CompletionItem>>();

    completions
}
