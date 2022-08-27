use nvim_oxi::{self as nvim, opts::OnBytesArgs};

#[derive(Default, Debug)]
pub struct CompletionContext {
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
    // id: RevId,
}

impl CompletionContext {
    /// TODO: docs
    #[inline(always)]
    pub fn current_line(&self) -> &str {
        &self.line
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

    pub fn ch(&self) -> char {
        'a'
    }

    #[inline]
    pub(crate) fn new(line: String, cursor: usize) -> Self {
        let prefix_offset = self::find_prefix(&line, cursor);
        Self { line, cursor, prefix_offset }
    }
}

impl TryFrom<&OnBytesArgs> for CompletionContext {
    type Error = nvim::Error;

    fn try_from(
        &(
            _,
            ref buf,
            _,
            start_row,
            start_col,
            _,
            _,
            _,
            bytes_deleted,
            _,
            _,
            bytes_added,
        ): &OnBytesArgs,
    ) -> Result<Self, Self::Error> {
        let col = start_col + if bytes_deleted != 0 { 0 } else { bytes_added };

        let line = buf
            .get_lines(start_row, start_row + 1, true)?
            .next()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        Ok(CompletionContext::new(line, col))
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
