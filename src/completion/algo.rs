use super::CompletionItem;

pub fn get_matched_prefix(line: &str, bytes_before_cursor: usize) -> &'_ str {
    let bytes_to_take = line[..bytes_before_cursor]
        .chars()
        .rev()
        .take_while(|&char| !char.is_whitespace())
        .collect::<String>()
        .len();

    &line[(bytes_before_cursor - bytes_to_take)..bytes_before_cursor]
}

pub fn complete(matched_prefix: &str) -> Vec<CompletionItem> {
    if matched_prefix.is_empty() {
        return Vec::new();
    }

    let entries = ["foo", "bar", "baz", "bazooka"];

    entries
        .iter()
        .filter(|&&entry| {
            entry.starts_with(matched_prefix) && entry != matched_prefix
        })
        .map(|entry| CompletionItem::new(entry.to_string(), matched_prefix))
        .collect::<Vec<CompletionItem>>()
}

#[cfg(test)]
mod tests {
    use super::get_matched_prefix;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // `|`
    fn empty_line() {
        assert_eq!("", get_matched_prefix("", 0))
    }

    #[test]
    // `|foo`
    fn cursor_at_beginning_of_line() {
        assert_eq!("", get_matched_prefix("foo", 0))
    }

    #[test]
    // ` ⇥|foo`
    fn only_whitespace_before_cursor() {
        assert_eq!("", get_matched_prefix(" \tfoo", 2))
    }

    #[test]
    // `foo |bar`
    fn cursor_before_word() {
        assert_eq!("", get_matched_prefix("foo bar", 4))
    }

    #[test]
    // `foo | bar`
    fn cursor_between_spaces() {
        assert_eq!("", get_matched_prefix("foo  bar", 4))
    }

    #[test]
    // `foo⇥|⇥bar`
    fn cursor_between_tabs() {
        assert_eq!("", get_matched_prefix("foo\t\tbar", 4))
    }

    #[test]
    // `foo|`
    fn cursor_end_of_word() {
        assert_eq!("foo", get_matched_prefix("foo", 3))
    }

    #[test]
    // `foo|bar`
    fn cursor_inside_word() {
        assert_eq!("foo", get_matched_prefix("foobar", 3))
    }

    #[test]
    // `fö|ö` (every `ö` is 2 bytes long)
    fn cursor_inside_word_multibyte_chars() {
        assert_eq!("fö", get_matched_prefix("föö", 3))
    }
}
