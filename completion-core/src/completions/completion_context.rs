#[derive(Debug)]
pub struct CompletionContext {
    pub(crate) line: LineContext,
}

impl CompletionContext {
    pub fn ch(&self) -> char {
        'a'
    }

    pub(crate) fn new(cursor: LineContext) -> Self {
        Self { line: cursor }
    }
}

#[derive(Debug)]
pub struct LineContext {
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

impl LineContext {
    /// TODO: docs
    #[inline(always)]
    pub fn current_line(&self) -> &str {
        &self.line
    }

    /// TODO: docs
    #[inline(always)]
    pub fn line_up_to_cursor(&self) -> &str {
        &self.line[..self.col - self.len_prefix]
    }

    /// TODO: docs
    #[inline(always)]
    pub fn line_from_cursor_to_end(&self) -> &str {
        let postfix = self::find_postfix(&self.line, self.col);
        &self.line[self.col + postfix..]
    }

    #[inline]
    pub(crate) fn new(row: usize, col: usize, line: String) -> Self {
        let len_prefix = self::find_prefix(&line, col);
        Self { row, col, line, len_prefix }
    }

    #[inline]
    pub(crate) fn is_at_eol(&self) -> bool {
        self.line.len() == self.col
    }

    /// TODO: docs
    #[inline(always)]
    pub(crate) fn matched_prefix(&self) -> &str {
        &self.line[self.col - self.len_prefix..self.col]
    }
}

/*

- helix
/helix-core/src/movement.rs -> is_word_boundary
/helix-core/src/chars.rs -> categorize_char


:h iskeyword
:lua =vim.api.nvim_buf_get_option(0, "iskeyword")

```lua
-- /runtime/lua/vim/lsp/handlers.lua L291
local line_to_cursor = "pub(crate) fn echo(msg:String"
local textMatch = vim.fn.match(line_to_cursor, '\\k*$')
local prefix = line_to_cursor:sub(textMatch + 1)
print(prefix) -- `String`
```
*/

/// TODO: docs
fn find_prefix(line: &str, col: usize) -> usize {
    debug_assert!(col <= line.len(), "col is {col} but line is {line:?}");

    const WORD_BOUNDARIES: &[u8] =
        &[b' ', b'.', b'\'', b'"', b'\t', b'(', b')', b'[', b']', b'{', b'}'];

    for (idx, byte) in line[..col].bytes().rev().enumerate() {
        if WORD_BOUNDARIES.contains(&byte) {
            return idx;
        }
    }

    col
}

/// TODO: docs
fn find_postfix(line: &str, col: usize) -> usize {
    debug_assert!(col <= line.len());

    const WORD_BOUNDARIES: &[u8] =
        &[b' ', b'.', b'\'', b'"', b'\t', b'(', b')', b'[', b']', b'{', b'}'];

    for (idx, byte) in line[col..].bytes().enumerate() {
        if WORD_BOUNDARIES.contains(&byte) {
            return idx;
        }
    }

    line.len() - col
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

#[cfg(test)]
mod postfix_tests {
    /// See doc comment above about `prefix_tests::find_prefix`.
    fn find_postfix(line: &str, col: usize) -> &str {
        let postfix = super::find_postfix(line, col);
        &line[col + postfix..]
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
