#[derive(Debug)]
pub struct CompletionItem {
    pub(crate) text: String,
}

impl CompletionItem {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
