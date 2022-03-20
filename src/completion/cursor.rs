use mlua::prelude::LuaResult;
use neovim::Api;

#[derive(Debug)]
pub struct Cursor {
    /// Number of bytes between the start of the line and the cursor.
    pub at_bytes: u32,

    /// The text in the row the cursor is currently on.
    pub line: String,

    /// Number of bytes before the cursor that are currently being matched to
    /// find a completion.
    pub matched_bytes: u32,

    /// The row the cursor is currently on.
    pub row: u32,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            at_bytes: 0,
            line: "".to_string(),
            matched_bytes: 0,
            row: 0,
        }
    }
}

impl Cursor {
    pub fn is_at_eol(&self) -> bool {
        self.at_bytes as usize == self.line.len()
    }

    pub fn update_at_bytes(&mut self, api: &Api) -> LuaResult<()> {
        self.at_bytes = api.win_get_cursor(0)?.1;
        Ok(())
    }

    pub fn update_line(&mut self, api: &Api) -> LuaResult<()> {
        self.line = api.get_current_line()?;
        Ok(())
    }

    pub fn update_matched_bytes(&mut self) {
        self.matched_bytes =
            get_matched_bytes(&self.line, self.at_bytes as usize)
                .try_into()
                .unwrap();
    }
}

fn get_matched_bytes(line: &str, bytes_before_cursor: usize) -> usize {
    line[..bytes_before_cursor]
        .bytes()
        .rev()
        .take_while(|&byte| !byte.is_ascii_whitespace())
        .count()
}

#[cfg(test)]
mod tests {
    use super::get_matched_bytes;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // `|`
    fn empty_line() {
        assert_eq!("".len(), get_matched_bytes("", 0))
    }

    #[test]
    // `|foo`
    fn cursor_at_beginning_of_line() {
        assert_eq!("".len(), get_matched_bytes("foo", 0))
    }

    #[test]
    // ` ⇥|foo`
    fn only_whitespace_before_cursor() {
        assert_eq!("".len(), get_matched_bytes(" \tfoo", 2))
    }

    #[test]
    // `foo |bar`
    fn cursor_before_word() {
        assert_eq!("".len(), get_matched_bytes("foo bar", 4))
    }

    #[test]
    // `foo | bar`
    fn cursor_between_spaces() {
        assert_eq!("".len(), get_matched_bytes("foo  bar", 4))
    }

    #[test]
    // `foo⇥|⇥bar`
    fn cursor_between_tabs() {
        assert_eq!("".len(), get_matched_bytes("foo\t\tbar", 4))
    }

    #[test]
    // `foo|`
    fn cursor_end_of_word() {
        assert_eq!("foo".len(), get_matched_bytes("foo", 3))
    }

    #[test]
    // `foo|bar`
    fn cursor_inside_word() {
        assert_eq!("foo".len(), get_matched_bytes("foobar", 3))
    }

    #[test]
    // `fö|ö` (every `ö` is 2 bytes long)
    fn cursor_inside_word_multibyte_chars() {
        assert_eq!("fö".len(), get_matched_bytes("föö", 3))
    }
}
