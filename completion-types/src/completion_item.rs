#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub text: String,

    /// TODO: docs
    pub filter_text: Option<String>,
    // /// TODO: docs
    // pub sort_text: String,

    // /// TODO: docs
    // pub insert_text: String,

    // /// TODO: docs
    // pub text_edit: String,
}

impl CompletionItem {
    pub fn builder() -> CompletionItemBuilder {
        CompletionItemBuilder::new()
    }

    pub fn filter_text(&self) -> &str {
        self.filter_text.as_ref().unwrap_or(&self.text)
    }
}

/// TODO: docs
#[derive(Debug, Clone)]
pub struct CompletionItemBuilder {
    item: Option<CompletionItem>,
}

impl CompletionItemBuilder {
    /// TODO: docs
    pub fn new() -> Self {
        let item = CompletionItem { text: "".to_owned(), filter_text: None };

        Self { item: Some(item) }
    }

    /// TODO: docs
    pub fn text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.item.as_mut().unwrap().text = text.into();
        self
    }

    /// TODO: docs
    pub fn filter_text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.item.as_mut().unwrap().filter_text = Some(text.into());
        self
    }

    /// TODO: docs
    pub fn build(&mut self) -> CompletionItem {
        self.item.take().unwrap()
    }
}
