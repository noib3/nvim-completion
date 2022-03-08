use mlua::Result;

use crate::completion::{self, CompletionItem};
use crate::Nvim;

// TODO: maybe rename this to a `LineState` and remove `completion_items`?
pub struct CompletionState {
    /// Number of bytes before (usually to be read as left-of, except for
    /// right-to-left languages).
    pub bytes_before_cursor: usize,

    /// The completion candidates computed by `completion::algo::complete`.
    pub completion_items: Vec<CompletionItem>,

    /// The text in the line at the current cursor position.
    pub current_line: String,

    /// The string we're using to find completion candidates.
    pub matched_prefix: String,
}

impl CompletionState {
    pub fn new() -> Self {
        CompletionState {
            current_line: "".to_string(),
            bytes_before_cursor: 0,
            matched_prefix: "".to_string(),
            completion_items: Vec::new(),
        }
    }
}

impl CompletionState {
    pub fn cursor_is_at_eol(&self) -> bool {
        self.bytes_before_cursor == self.current_line.len()
    }

    pub fn update_bytes_before_cursor(&mut self, nvim: &Nvim) -> Result<()> {
        self.bytes_before_cursor = nvim.win_get_cursor(0)?.1;
        Ok(())
    }

    pub fn update_current_line(&mut self, nvim: &Nvim) -> Result<()> {
        self.current_line = nvim.get_current_line()?;
        Ok(())
    }

    pub fn update_matched_prefix(&mut self) -> Result<()> {
        self.matched_prefix = completion::get_matched_prefix(
            &self.current_line,
            self.bytes_before_cursor,
        )
        .to_string();
        Ok(())
    }
}
