#[derive(Debug)]
pub struct Position {
    // TODO: docs
    pub row: u32,

    // TODO: docs
    pub col: usize,

    // offset: u32,

    // TODO: docs
    pub line: String,
}

impl Position {
    #[inline]
    pub fn new<L>(row: u32, character: u32, line: L) -> Self
    where
        L: Into<String>,
    {
        Self { row, col: character as _, line: line.into() }
    }
}
