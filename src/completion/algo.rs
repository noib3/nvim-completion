use itertools::Itertools;

use crate::completion::CompletionItem;

pub fn complete(line: &str, bytes_before_cursor: u64) -> Vec<CompletionItem> {
    let prefix = get_prefix(line, bytes_before_cursor);

    if prefix.is_empty() {
        return Vec::new();
    }

    let entries = [
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ];

    entries
        .into_iter()
        .filter(|entry| entry.starts_with(&prefix) && entry != &prefix)
        .map(|entry| CompletionItem::new(entry, &prefix))
        .collect::<Vec<CompletionItem>>()
}

fn get_prefix(line: &str, bytes_before_cursor: u64) -> String {
    line[..bytes_before_cursor as usize]
        .chars()
        .rev()
        .take_while(|&char| !char.is_whitespace())
        // I wish I could just put a `.rev()` here and end the chain after the
        // next `.join("")`. Unfortunately `TakeWhile` doesn't implement
        // `DoulbeEndedIterator`, which is a trait bound needed by `.rev()`.
        .join("")
        .chars()
        .rev()
        .join("")
}
