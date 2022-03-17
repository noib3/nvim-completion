use mlua::Result;
use neovim::Api;

#[derive(Debug)]
pub struct Buffer {
    /// The row the cursor is currently on.
    pub row: usize,

    /// Number of bytes before (i.e. left-of) the current cursor position.
    pub at_bytes: usize,

    /// The text in the row the cursor is currently on.
    pub line: String,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            row: 0,
            at_bytes: 0,
            line: "".to_string(),
        }
    }
}

impl Buffer {
    pub fn cursor_is_at_eol(&self) -> bool {
        self.at_bytes == self.line.len()
    }

    pub fn get_bytes_before_cursor(&mut self, api: &Api) -> Result<()> {
        self.at_bytes = api.win_get_cursor(0)?.1;
        Ok(())
    }

    pub fn get_text(&mut self, api: &Api) -> Result<()> {
        self.line = api.get_current_line()?;
        Ok(())
    }
}
