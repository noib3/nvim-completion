use std::collections::HashMap;

use super::{CompletionConfig, CompletionItem};
use crate::sources::SourceId;

type CompletionTimes = [u16; 1024];

/// TODO: docs
#[derive(Default)]
pub(crate) struct CompletionState {
    completions: HashMap<SourceId, Vec<CompletionItem>>,
    // displayed_completions: Vec<(SourceId, )
    config: CompletionConfig,

    selected_idx: Option<usize>,

    statistics: HashMap<SourceId, CompletionTimes>,
}

impl CompletionState {
    #[inline]
    pub(crate) fn init(&mut self, config: CompletionConfig) {
        self.config = config;
    }
}
