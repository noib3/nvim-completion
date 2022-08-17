use nvim_oxi::api::Buffer;
// use ropey::Rope;

#[derive(Default, Debug)]
pub struct CompletionContext {
    // rope: Rope,
    buf: Option<Buffer>,

    /// The line the cursor is currently on.
    line: String,

    /// Byte offset of the cursor position in the current line. For example, if
    /// the current line is `foo ba|r baz`, where `|` indicates the cursor,
    /// then `cursor` will be equal to `6`.
    ///
    /// INVARIANT: always between `0` and `line.len()`.
    cursor: usize,

    /// TODO: docs
    prefix_offset: usize,
}

impl CompletionContext {
    /// Initializes and returns a new completion context for a specific buffer.
    pub(crate) fn new(buf: Buffer) -> Self {
        Self { buf: Some(buf), ..Default::default() }
    }

    pub fn ch(&self) -> char {
        'a'
    }

    /// TODO: docs
    pub(crate) fn apply_change(&mut self) {
        self.prefix_offset = self::find_prefix(&self.line, self.cursor);
    }

    /// Returns a reference to the [`Buffer`] associated to this context.
    #[inline]
    pub(crate) fn buf(&self) -> &Buffer {
        self.buf.as_ref().unwrap()
    }

    /// TODO: docs
    pub fn file_path(&self) -> &std::path::Path {
        todo!()
    }

    /// TODO: docs
    #[inline(always)]
    pub fn line_up_to_cursor(&self) -> &str {
        &self.line[..self.prefix_offset]
    }

    /// TODO: docs
    #[inline(always)]
    pub fn line_from_cursor_to_end(&self) -> &str {
        let offset = self::find_postfix(&self.line, self.cursor);
        &self.line[offset..]
    }
}

/// TODO: docs
const WORD_BOUNDARIES: &[u8] =
    &[b' ', b'.', b'\'', b'"', b'\t', b'(', b')', b'[', b']', b'{', b'}'];

/// TODO: docs
fn find_prefix(line: &str, cursor: usize) -> usize {
    // naive: walk back until you find a word boundary character counting the
    // steps you take. Once you find one return `cursor - steps`.
    debug_assert!(cursor <= line.len());

    let steps = line[..cursor]
        .bytes()
        .rev()
        .take_while(|byte| !WORD_BOUNDARIES.contains(byte))
        .count();

    // Imperative solution.
    //
    // let mut steps = 0;
    // for byte in line[..cursor].bytes().rev() {
    //     if WORD_BOUNDARIES.contains(&byte) {
    //         break;
    //     }
    //     steps += 1;
    // }

    cursor - steps
}

/// TODO: docs
fn find_postfix(line: &str, cursor: usize) -> usize {
    debug_assert!(cursor <= line.len());

    let steps = line[cursor..]
        .bytes()
        .take_while(|byte| !WORD_BOUNDARIES.contains(byte))
        .count();

    // Imperative solution.
    //
    // let mut steps = 0;
    // for byte in line[cursor..].bytes() {
    //     if WORD_BOUNDARIES.contains(&byte) {
    //         break;
    //     }
    //     steps += 1;
    // }

    cursor + steps
}

#[cfg(test)]
mod prefix_tests {
    /// Shadows the actual `find_prefix` function in the outer module to return
    /// the prefix as a string slice instead of as a byte offset. Should make
    /// `assert`s easier to read.
    fn find_prefix(line: &str, cursor: usize) -> &str {
        let offset = super::find_prefix(line, cursor);
        &line[..offset]
    }

    #[test]
    fn foo() {
        let p = find_prefix("foo", 3);
        assert_eq!("", p)
    }

    #[test]
    fn foo_dot_bar() {
        let p = find_prefix("foo.bar", 6);
        assert_eq!("foo.", p)
    }
}

#[cfg(test)]
mod postfix_tests {
    /// See doc comment above about `prefix_tests::find_prefix`.
    fn find_postfix(line: &str, cursor: usize) -> &str {
        let offset = super::find_postfix(line, cursor);
        &line[offset..]
    }

    #[test]
    fn foo() {
        let p = find_postfix("foo", 0);
        assert_eq!("", p)
    }

    #[test]
    fn foo_dot_bar() {
        let p = find_postfix("foo.bar", 2);
        assert_eq!(".bar", p)
    }
}
