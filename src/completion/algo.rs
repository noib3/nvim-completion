use super::CompletionItem;

pub fn complete(
    line: &str,
    bytes_before_cursor: usize,
) -> Vec<CompletionItem> {
    let matched_prefix_len = get_matched_prefix(line, bytes_before_cursor);

    if matched_prefix_len == 0 {
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

    let matched_prefix =
        &line[(bytes_before_cursor - matched_prefix_len)..bytes_before_cursor];

    let completions = entries
        .iter()
        .filter(|&&entry| {
            entry.0.starts_with(matched_prefix) && entry.0 != matched_prefix
        })
        .map(|entry| {
            CompletionItem::new(
                entry.0.to_string(),
                entry.1.map(|d| d.into()),
                matched_prefix_len,
            )
        })
        .collect::<Vec<CompletionItem>>();

    completions
}

fn get_matched_prefix(line: &str, bytes_before_cursor: usize) -> usize {
    line[..bytes_before_cursor]
        .bytes()
        .rev()
        .take_while(|&byte| !byte.is_ascii_whitespace())
        .count()

    // &line[(bytes_before_cursor - bytes_to_take)..bytes_before_cursor]
}

#[cfg(test)]
mod tests {
    use super::get_matched_prefix;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // `|`
    fn empty_line() {
        assert_eq!("".len(), get_matched_prefix("", 0))
    }

    #[test]
    // `|foo`
    fn cursor_at_beginning_of_line() {
        assert_eq!("".len(), get_matched_prefix("foo", 0))
    }

    #[test]
    // ` ⇥|foo`
    fn only_whitespace_before_cursor() {
        assert_eq!("".len(), get_matched_prefix(" \tfoo", 2))
    }

    #[test]
    // `foo |bar`
    fn cursor_before_word() {
        assert_eq!("".len(), get_matched_prefix("foo bar", 4))
    }

    #[test]
    // `foo | bar`
    fn cursor_between_spaces() {
        assert_eq!("".len(), get_matched_prefix("foo  bar", 4))
    }

    #[test]
    // `foo⇥|⇥bar`
    fn cursor_between_tabs() {
        assert_eq!("".len(), get_matched_prefix("foo\t\tbar", 4))
    }

    #[test]
    // `foo|`
    fn cursor_end_of_word() {
        assert_eq!("foo".len(), get_matched_prefix("foo", 3))
    }

    #[test]
    // `foo|bar`
    fn cursor_inside_word() {
        assert_eq!("foo".len(), get_matched_prefix("foobar", 3))
    }

    #[test]
    // `fö|ö` (every `ö` is 2 bytes long)
    fn cursor_inside_word_multibyte_chars() {
        assert_eq!("fö".len(), get_matched_prefix("föö", 3))
    }
}
