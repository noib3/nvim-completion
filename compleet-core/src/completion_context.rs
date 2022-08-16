// use nvim_oxi::api::Buffer;
// use ropey::Rope;

pub struct CompletionContext {
    // buffer: Buffer,
    // rope: Rope,
    ch: char,
}

impl CompletionContext {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }

    pub fn ch(&self) -> char {
        self.ch
    }

    pub fn line_up_to_cursor(&self) -> &str {
        todo!()
    }

    pub fn line_from_cursor_to_end(&self) -> &str {
        todo!()
    }
}
