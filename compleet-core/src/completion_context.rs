use std::ops::Deref;

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct CompletionContext {
    pub(crate) cursor: Cursor,
}

impl CompletionContext {
    pub fn ch(&self) -> char {
        'a'
    }

    pub(crate) fn new(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Deref for CompletionContext {
    type Target = Cursor;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}
