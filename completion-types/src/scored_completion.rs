use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::CompletionItem;

pub type Score = i64;

#[derive(Debug)]
pub struct ScoredCompletion {
    pub item: Arc<CompletionItem>,
    pub score: Score,
    pub matched_bytes: Vec<usize>,
}

impl ScoredCompletion {
    pub fn matched_ranges(&self) -> Vec<RangeInclusive<usize>> {
        if self.matched_bytes.is_empty() {
            return vec![];
        }

        let mut ranges = Vec::new();

        let mut start = self.matched_bytes[0];
        let mut current = start;

        for &char_idx in self.matched_bytes.iter().skip(1) {
            if char_idx != current + 1 {
                ranges.push(start..=current + 1);
                start = char_idx;
                current = start;
            }

            current += 1;
        }

        let last = self.matched_bytes.last().unwrap();

        if ranges.is_empty() || *ranges.last().unwrap().end() != last + 1 {
            ranges.push(start..=last + 1);
        }

        ranges
    }
}
