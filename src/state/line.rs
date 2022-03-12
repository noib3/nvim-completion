use mlua::Result;
use neovim::Api;

use crate::completion;

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
        self.matched_prefix = completion::get_matched_prefix(
            &self.text,
            self.bytes_before_cursor,
        )
        .to_string();
        Ok(())
    }
}
