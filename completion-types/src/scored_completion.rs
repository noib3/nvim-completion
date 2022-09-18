use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::CompletionItem;

pub type Score = i64;

pub struct ScoredCompletion {
    pub item: Arc<CompletionItem>,
    pub score: Score,
    pub matched_bytes: Vec<usize>,
}

impl ScoredCompletion {
    pub fn matched_ranges(&self) -> Vec<RangeInclusive<usize>> {
        todo!()
    }
}
