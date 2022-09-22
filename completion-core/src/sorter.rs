use std::sync::Arc;

use completion_types::{
    CompletionItem,
    CompletionRequest,
    Document,
    Position,
    ScoredCompletion,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;

use crate::PositionExt;

type Score = i64;

/// TODO: docs
pub(crate) fn sort(
    items: Vec<Arc<CompletionItem>>,
    request: &CompletionRequest,
) -> Vec<ScoredCompletion> {
    let matcher = SkimMatcherV2::default();

    let mut completions = items
        .into_par_iter()
        .filter_map(|item| {
            let (score, matched_bytes) = score_completion(
                &item,
                &request.document,
                &request.position,
                &matcher,
            )?;

            Some(ScoredCompletion { item, score, matched_bytes })
        })
        .collect::<Vec<_>>();

    completions.par_sort_by(|a, b| b.score.cmp(&a.score));

    completions
}

/// TODO: docs
fn score_completion<M: FuzzyMatcher>(
    item: &CompletionItem,
    _document: &Document,
    position: &Position,
    matcher: &M,
) -> Option<(Score, Vec<usize>)> {
    matcher.fuzzy_indices(&item.text, position.matched_prefix())
}
