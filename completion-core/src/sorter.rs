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

use crate::PositionExt;

type Score = i64;

const ITEMS_NUM_THRESHOLD: usize = 1024;
// const NUM_THREADS: u8 = 8;

pub(crate) fn sort(
    items: Vec<Arc<CompletionItem>>,
    request: Arc<CompletionRequest>,
    // document: Arc<Document>,
    // position: Arc<Position>,
) -> Vec<ScoredCompletion> {
    let matcher = Arc::new(SkimMatcherV2::default());

    let mut completions = Vec::new();

    let mut handles = items
        .chunks(ITEMS_NUM_THRESHOLD)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            let mach = Arc::clone(&matcher);
            let req = Arc::clone(&request);
            // let pos = Arc::clone(&position);
            std::thread::spawn(move || score_chunk(&chunk, &req, &*mach))
        })
        .collect::<Vec<_>>();

    while let Some(handle) = handles.pop() {
        completions.extend(handle.join().unwrap());
    }

    // while !handles.is_empty() {
    //     compls.extend(handles.pop())
    // }

    // let mut items = items
    //     .iter()
    //     .filter_map(|completion| {
    //         let (score, matched_bytes) =
    //             score(completion, document, position, &*matcher)?;

    //         Some(ScoredCompletion {
    //             item: Arc::clone(&completion),
    //             score,
    //             matched_bytes,
    //         })
    //     })
    //     .collect::<Vec<_>>();

    completions.sort_by(|a, b| b.score.cmp(&a.score));

    completions
}

fn score_chunk<M: FuzzyMatcher>(
    chunk: &[Arc<CompletionItem>],
    request: &CompletionRequest,
    matcher: &M,
) -> Vec<ScoredCompletion> {
    chunk
        .iter()
        .filter_map(|completion| {
            let (score, matched_bytes) = score_completion(
                completion,
                &request.document,
                &request.position,
                matcher,
            )?;

            Some(ScoredCompletion {
                item: Arc::clone(&completion),
                score,
                matched_bytes,
            })
        })
        .collect::<Vec<_>>()
}

fn score_completion<M: FuzzyMatcher>(
    item: &CompletionItem,
    _document: &Document,
    position: &Position,
    matcher: &M,
) -> Option<(Score, Vec<usize>)> {
    matcher.fuzzy_indices(&item.text, position.matched_prefix())
}
