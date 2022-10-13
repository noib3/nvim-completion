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
        to_ranges(&self.matched_bytes)
    }
}

/// TODO: docs
pub(crate) fn to_ranges(v: &[usize]) -> Vec<RangeInclusive<usize>> {
    if v.is_empty() {
        return vec![];
    }

    let mut ranges = Vec::new();

    let mut start = v[0];
    let mut current = start;

    for &char_idx in v.iter().skip(1) {
        if char_idx != current + 1 {
            ranges.push(start..=current + 1);
            start = char_idx;
            current = start;
            continue;
        }

        current += 1;
    }

    let last = v.last().unwrap();

    if ranges.is_empty() || *ranges.last().unwrap().end() != last + 1 {
        ranges.push(start..=last + 1);
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ranges() {
        let v = [];
        assert!(to_ranges(&v).is_empty());

        let v = [0, 1, 2];
        assert_eq!(&[0..=3], &*to_ranges(&v));

        let v = [0, 1, 3];
        assert_eq!(&[0..=2, 3..=4], &*to_ranges(&v));

        let v = [0, 2];
        assert_eq!(&[0..=1, 2..=3], &*to_ranges(&v));

        let v = [0, 1, 6, 8, 9];
        assert_eq!(&[0..=2, 6..=7, 8..=10], &*to_ranges(&v));
    }
}
