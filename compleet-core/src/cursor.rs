pub(crate) struct Cursor {
    /// TODO: docs
    pub(crate) row: usize,

    /// Number of bytes between the start of the line and the current cursor
    /// position. For example, if the current line is `foo ba|r baz`, where
    /// `|` indicates the cursor, then `col` will be equal to `6`.
    ///
    /// INVARIANT: always between `0` and `line.len()`.
    pub(crate) col: usize,

    /// The line the cursor is currently on.
    ///
    /// INVARIANT: doesn't contain any newline characters.
    pub(crate) line: String,

    /// TODO: docs
    pub(crate) len_prefix: usize,
}

impl Cursor {
    #[inline]
    pub(crate) fn new(row: usize, col: usize, line: String) -> Self {
        let len_prefix = self::find_prefix(&line, col);
        Self { row, col, line, len_prefix }
    }

    pub fn is_at_eol(&self) -> bool {
        self.line.len() == self.col
    }
}

/// TODO: docs
fn find_prefix(line: &str, col: usize) -> usize {
    debug_assert!(col <= line.len());

    const WORD_BOUNDARIES: &[u8] =
        &[b' ', b'.', b'\'', b'"', b'\t', b'(', b')', b'[', b']', b'{', b'}'];

    for (idx, byte) in line[..col].bytes().rev().enumerate() {
        if WORD_BOUNDARIES.contains(&byte) {
            return idx;
        }
    }

    col
}

#[cfg(test)]
mod prefix_tests {
    /// Shadows the actual `find_prefix` function in the outer module to return
    /// the prefix as a string slice instead of as a byte offset. Should make
    /// `assert`s easier to read.
    fn find_prefix(line: &str, col: usize) -> &str {
        let prefix = super::find_prefix(line, col);
        &line[..col - prefix]
    }

    #[test]
    fn foo() {
        let p = find_prefix("foo", 3);
        assert_eq!("", p)
    }

    #[test]
    fn foo_dot() {
        let p = find_prefix("foo.", 4);
        assert_eq!("foo.", p)
    }

    #[test]
    fn foo_dot_bar() {
        let p = find_prefix("foo.bar", 6);
        assert_eq!("foo.", p)
    }
}
