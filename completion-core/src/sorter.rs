use std::sync::Arc;

use completion_types::{
    CompletionItem,
    CompletionRequest,
    Document,
    ScoredCompletion,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;

use crate::PositionExt;

type Score = i64;

/// Sorts a list of completion items against a specific request, filtering out
/// the completions that don't match the request.
pub(crate) fn sort(
    items: Vec<Arc<CompletionItem>>,
    request: &CompletionRequest,
) -> Vec<ScoredCompletion> {
    let matcher = SkimMatcherV2::default();

    let prefix = request.position.matched_prefix();

    let mut completions = items
        .into_par_iter()
        .filter_map(|item| {
            let (score, matched_bytes) =
                score_completion(&matcher, &item, &request.document, &prefix)?;

            Some(ScoredCompletion { item, score, matched_bytes })
        })
        .collect::<Vec<_>>();

    completions.par_sort_by(|a, b| b.score.cmp(&a.score));

    completions
}

/// Scores a single completion item against a prefix, which is usually the word
/// under the cursor (delimited by some word-boundary rules).
///
/// If the completion doesn't match the prefix a `None` value will be returned.
///
/// If it matches it also returns a vector containing the characters of the
/// completion item that are matched by the prefix.
fn score_completion<M: FuzzyMatcher>(
    matcher: &M,
    completion: &CompletionItem,
    _document: &Document,
    prefix: &str,
) -> Option<(Score, Vec<usize>)> {
    matcher.fuzzy_indices(completion.filter_text(), prefix)
}
