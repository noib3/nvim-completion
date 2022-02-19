use crate::{completion::CompletionItem, Nvim};

pub async fn insert_completion(
    _nvim: &Nvim,
    _current_line: &str,
    _bytes_before_cursor: u64,
    _completion: &CompletionItem,
) {
    todo![]
}
