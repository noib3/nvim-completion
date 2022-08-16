/// TODO: docs
#[derive(Debug)]
pub struct CompletionItem {
    pub(crate) text: String,
}

/// TODO: docs
#[derive(Debug, Clone)]
pub struct CompletionItemBuilder {
    text: Option<String>,
}

impl CompletionItemBuilder {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: Some(text.into()) }
    }

    pub fn build(mut self) -> CompletionItem {
        CompletionItem { text: self.text.take().unwrap() }
    }
}
