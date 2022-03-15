use mlua::Result;
use neovim::Api;

#[derive(Debug)]
pub struct Line {
    /// Number of bytes before (usually to be read as left-of, except for
    /// right-to-left languages).
    pub bytes_before_cursor: usize,

    /// The text in the line at the current cursor position.
    pub text: String,

    /// The string we're using to find completion candidates.
    pub matched_prefix: String,
}

impl Line {
    pub fn new() -> Self {
        Line {
            bytes_before_cursor: 0,
            text: "".to_string(),
            matched_prefix: "".to_string(),
        }
    }
}

impl Line {
    pub fn cursor_is_at_eol(&self) -> bool {
        self.bytes_before_cursor == self.text.len()
    }

    pub fn update_bytes_before_cursor(&mut self, api: &Api) -> Result<()> {
        self.bytes_before_cursor = api.win_get_cursor(0)?.1;
        Ok(())
    }

    pub fn update_text(&mut self, api: &Api) -> Result<()> {
        self.text = api.get_current_line()?;
        Ok(())
    }

    pub fn update_matched_prefix(&mut self) -> Result<()> {
        self.matched_prefix =
            get_matched_prefix(&self.text, self.bytes_before_cursor)
                .to_string();
        Ok(())
    }
}

fn get_matched_prefix(line: &str, bytes_before_cursor: usize) -> &'_ str {
    let bytes_to_take = line[..bytes_before_cursor]
        .bytes()
        .rev()
        .take_while(|&byte| !byte.is_ascii_whitespace())
        .count();

    &line[(bytes_before_cursor - bytes_to_take)..bytes_before_cursor]
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
