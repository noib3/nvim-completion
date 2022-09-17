use crate::CompletionItem;

#[derive(Debug, Clone)]
pub struct CompletionList {
    pub items: Vec<CompletionItem>,
    pub is_complete: bool,
}
