#[derive(Debug)]
pub struct Cursor {
    /// Number of bytes between the start of the line and the cursor.
    pub at_bytes: u32,

    /// The text in the row the cursor is currently on.
    pub line: String,

    /// The row the cursor is currently on.
    pub row: u32,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            at_bytes: 0,
            line: "".to_string(),
            row: 0,
        }
    }
}

impl Cursor {
    /// TODO: docs
    pub fn is_at_eol(&self) -> bool {
        self.at_bytes as usize == self.line.len()
    }

    /// TODO: docs
    pub fn _is_at_sol(&self) -> bool {
        self.at_bytes == 0
    }

    /// TODO: docs
    fn non_whitespace_bytes_pre(&self) -> usize {
        self.line[..self.at_bytes as usize]
            .bytes()
            .rev()
            .take_while(|&byte| !byte.is_ascii_whitespace())
            .count()
    }

    /// TODO: docs
    fn _non_whitespace_bytes_post(&self) -> usize {
        self.line[self.at_bytes as usize..]
            .bytes()
            .take_while(|&byte| !byte.is_ascii_whitespace())
            .count()
    }

    /// TODO: docs
    pub fn _word(&self) -> &'_ str {
        &self.line[self.at_bytes as usize - self.non_whitespace_bytes_pre()
            ..self.at_bytes as usize + self._non_whitespace_bytes_post()]
    }

    /// TODO: docs
    pub fn word_pre(&self) -> &'_ str {
        &self.line[self.at_bytes as usize - self.non_whitespace_bytes_pre()
            ..self.at_bytes as usize]
    }

    /// TODO: docs
    pub fn _word_post(&self) -> &'_ str {
        &self.line[self.at_bytes as usize
            ..self.at_bytes as usize + self._non_whitespace_bytes_post()]
    }
}

fn _get_matched_bytes(line: &str, bytes_before_cursor: usize) -> usize {
    line[..bytes_before_cursor]
        .bytes()
        .rev()
        .take_while(|&byte| !byte.is_ascii_whitespace())
        .count()
}

#[cfg(test)]
mod tests {
    use super::_get_matched_bytes;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // `|`
    fn empty_line() {
        assert_eq!("".len(), _get_matched_bytes("", 0))
    }

    #[test]
    // `|foo`
    fn cursor_at_beginning_of_line() {
        assert_eq!("".len(), _get_matched_bytes("foo", 0))
    }

    #[test]
    // ` ⇥|foo`
    fn only_whitespace_before_cursor() {
        assert_eq!("".len(), _get_matched_bytes(" \tfoo", 2))
    }

    #[test]
    // `foo |bar`
    fn cursor_before_word() {
        assert_eq!("".len(), _get_matched_bytes("foo bar", 4))
    }

    #[test]
    // `foo | bar`
    fn cursor_between_spaces() {
        assert_eq!("".len(), _get_matched_bytes("foo  bar", 4))
    }

    #[test]
    // `foo⇥|⇥bar`
    fn cursor_between_tabs() {
        assert_eq!("".len(), _get_matched_bytes("foo\t\tbar", 4))
    }

    #[test]
    // `foo|`
    fn cursor_end_of_word() {
        assert_eq!("foo".len(), _get_matched_bytes("foo", 3))
    }

    #[test]
    // `foo|bar`
    fn cursor_inside_word() {
        assert_eq!("foo".len(), _get_matched_bytes("foobar", 3))
    }

    #[test]
    // `fö|ö` (every `ö` is 2 bytes long)
    fn cursor_inside_word_multibyte_chars() {
        assert_eq!("fö".len(), _get_matched_bytes("föö", 3))
    }
}
