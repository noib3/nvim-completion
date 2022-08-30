use std::collections::HashMap;

use super::{CompletionConfig, CompletionItem};
use crate::sources::SourceId;

type CompletionTimes = [u16; 1024];

/// TODO: docs
#[derive(Default)]
pub(crate) struct CompletionState {
    completions: Vec<(CompletionItem, bool)>,

    config: CompletionConfig,

    selected_idx: Option<usize>,

    statistics: HashMap<SourceId, CompletionTimes>,
}

impl CompletionState {
    #[inline]
    pub(crate) fn set_config(&mut self, config: CompletionConfig) {
        self.config = config;
    }
}
