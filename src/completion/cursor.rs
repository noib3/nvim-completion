use mlua::prelude::LuaResult;
use neovim::Api;

#[derive(Debug)]
pub struct Cursor {
    /// The row the cursor is currently on.
    pub row: u32,

    /// Number of bytes before (i.e. left-of) the current cursor position.
    pub bytes: u32,

    /// The text in the row the cursor is currently on.
    pub line: String,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            row: 0,
            bytes: 0,
            line: "".to_string(),
        }
    }
}

impl Cursor {
    pub fn is_at_eol(&self) -> bool {
        self.bytes as usize == self.line.len()
    }

    pub fn update_bytes(&mut self, api: &Api) -> LuaResult<()> {
        self.bytes = api.win_get_cursor(0)?.1;
        Ok(())
    }

    pub fn update_line(&mut self, api: &Api) -> LuaResult<()> {
        self.line = api.get_current_line()?;
        Ok(())
    }
}
