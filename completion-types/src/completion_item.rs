#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub text: String,
}

/// TODO: docs
#[derive(Debug, Clone)]
pub struct CompletionItemBuilder {
    text: Option<String>,
}

impl CompletionItemBuilder {
    /// TODO: docs
    #[inline(always)]
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: Some(text.into()) }
    }

    /// TODO: docs
    #[inline(always)]
    pub fn build(mut self) -> CompletionItem {
        CompletionItem { text: self.text.take().unwrap() }
    }
}
