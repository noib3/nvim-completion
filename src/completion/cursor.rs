// TODO: remove `Clone`
#[derive(Debug, Clone)]
pub struct Cursor {
    /// The number of bytes between the start of the line and the cursor.
    pub bytes: u32,

    /// The text in the row the cursor is currently on.
    pub line: String,

    /// The row the cursor is currently on.
    pub row: u32,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            bytes: 0,
            line: "".to_string(),
            row: 0,
        }
    }
}

impl Cursor {
    /// Whether the cursor is at the end of the line.
    pub fn is_at_eol(&self) -> bool { self.bytes as usize == self.line.len() }

    /// Whether the cursor is at the start of the line.
    pub fn _is_at_sol(&self) -> bool { self.bytes == 0 }

    /// The number of bytes between the cursor and the first whitespace
    /// character before it.
    fn non_whitespace_bytes_pre(&self) -> usize {
        self.line[..self.bytes as usize]
            .bytes()
            .rev()
            .take_while(|&byte| !byte.is_ascii_whitespace())
            .count()
    }

    /// The number of bytes between the cursor and the first whitespace
    /// character after it.
    fn _non_whitespace_bytes_post(&self) -> usize {
        self.line[self.bytes as usize..]
            .bytes()
            .take_while(|&byte| !byte.is_ascii_whitespace())
            .count()
    }

    /// The current word the cursor is embedded in, where a word is considered
    /// a collection of non-whitespace bytes.
    pub fn _word(&self) -> &'_ str {
        &self.line[self.bytes as usize - self.non_whitespace_bytes_pre()
            ..self.bytes as usize + self._non_whitespace_bytes_post()]
    }

    /// The part of the word the cursor is on that's before the cursor.
    pub fn word_pre(&self) -> &'_ str {
        &self.line[self.bytes as usize - self.non_whitespace_bytes_pre()
            ..self.bytes as usize]
    }

    /// The part of the word the cursor is on that's after the cursor.
    pub fn _word_post(&self) -> &'_ str {
        &self.line[self.bytes as usize
            ..self.bytes as usize + self._non_whitespace_bytes_post()]
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
    fn empty_line() { assert_eq!("".len(), _get_matched_bytes("", 0)) }

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
