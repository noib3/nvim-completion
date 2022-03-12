use super::CompletionItem;

pub fn complete(matched_prefix: &str) -> Vec<CompletionItem> {
    if matched_prefix.is_empty() {
        return Vec::new();
    }

    let entries = [
        ("foo", Some("A foo")),
        ("bar", Some("A bar")),
        ("baz", Some("A baz")),
        ("bam", None),
        ("bazooka", None),
        ("baroo", None),
        ("barometer", None),
        ("beard", None),
        ("bear", None),
        ("bamm", None),
        ("bamboozled", None),
        ("bambi", None),
    ];

    entries
        .iter()
        .filter(|&&entry| {
            entry.0.starts_with(matched_prefix) && entry.0 != matched_prefix
        })
        .map(|entry| {
            CompletionItem::new(
                entry.0.to_string(),
                entry.1.map(|d| d.into()),
                matched_prefix,
            )
        })
        .collect::<Vec<CompletionItem>>()
}
