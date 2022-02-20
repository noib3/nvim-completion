use itertools::Itertools;

use super::CompletionItem;

// TODO: docs
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

// TODO: is there a (possibly less elegant) way to do this w/o using
// `.join("")` twice? It creates a new `String` every time. I just need a
// string slice that w/ the same lifetime of `line`. No assignments should be
// needed.
//
// fn get_prefix<'a>(line: &'a str, bytes_before_cursor: u64) -> &'a str {
//
// TODO: docs
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

#[cfg(test)]
mod tests {
    use super::get_prefix;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // `|`
    fn empty_line() {
        assert_eq!("", get_prefix("", 0).as_str())
    }

    #[test]
    // `|foo`
    fn cursor_at_beginning_of_line() {
        assert_eq!("", get_prefix("foo", 0).as_str())
    }

    #[test]
    // ` ⇥|foo`
    fn only_whitespace_before_cursor() {
        assert_eq!("", get_prefix(" \tfoo", 2).as_str())
    }

    #[test]
    // `foo |bar`
    fn cursor_before_word() {
        assert_eq!("", get_prefix("foo bar", 4).as_str())
    }

    #[test]
    // `foo | bar`
    fn cursor_between_spaces() {
        assert_eq!("", get_prefix("foo  bar", 4).as_str())
    }

    #[test]
    // `foo⇥|⇥bar`
    fn cursor_between_tabs() {
        assert_eq!("", get_prefix("foo\t\tbar", 4).as_str())
    }

    #[test]
    // `foo|`
    fn cursor_end_of_word() {
        assert_eq!("foo", get_prefix("foo", 3).as_str())
    }

    #[test]
    // `foo|bar`
    fn cursor_inside_word() {
        assert_eq!("foo", get_prefix("foobar", 3).as_str())
    }

    #[test]
    // `fö|ö` (every `ö` is 2 bytes long)
    fn cursor_inside_word_multibyte_chars() {
        assert_eq!("fö", get_prefix("föö", 3).as_str())
    }
}
