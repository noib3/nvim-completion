//! TODO: docs

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::utils;
use crate::CompletionItem;

/// TODO: docs
pub(super) fn fuzzy_find(
    matched_prefix: &str,
    completions: &mut [CompletionItem],
) -> Vec<usize> {
    let matcher = SkimMatcherV2::default();

    let mut matches = completions
        .iter_mut()
        .enumerate()
        .filter_map(|(idx, completion)| {
            let (score, matched_chars) = matcher
                .fuzzy_indices(completion.matched_text(), matched_prefix)?;

            completion.set_matched_bytes(utils::to_ranges(&matched_chars));

            Some((idx, score))
        })
        .collect::<Vec<(usize, i64)>>();

    // Sort the results by decreasing score.
    matches.sort_by(|(_, score_a), (_, score_b)| score_b.cmp(&score_a));

    // Remove the score before returning.
    matches.into_iter().map(|(idx, _score)| idx).collect()
}
