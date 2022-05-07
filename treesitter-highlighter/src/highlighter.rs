use std::{clone, fmt};

use tree_sitter_highlight::{
    Highlight,
    HighlightConfiguration,
    HighlightEvent,
    Highlighter as OGHighlighter,
};

use crate::constants::{HIGHLIGHT_NAMES, TS_HLGROUPS};
use crate::generated::config_from_filetype;

pub struct Highlighter {
    config: HighlightConfiguration,
    highlighter: OGHighlighter,
}

// A dummy implementation of `Clone` just to satisfy the trait bounds of
// `Arc::make_mut`.
impl clone::Clone for Highlighter {
    fn clone(&self) -> Self {
        Self::from_filetype("").expect("this should never get cloned!")
    }
}

impl fmt::Debug for Highlighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Highlighter({:?})", self.config.language)
    }
}

impl Highlighter {
    pub fn from_filetype(ft: &str) -> Option<Self> {
        let mut config = config_from_filetype(ft)?;
        config.configure(HIGHLIGHT_NAMES);
        Some(Self { config, highlighter: OGHighlighter::new() })
    }
}

impl Highlighter {
    /// TODO: docs
    pub fn highlight(
        &mut self,
        text: &str,
    ) -> Vec<(std::ops::Range<usize>, &'static str)> {
        let mut events = self
            .highlighter
            .highlight(&self.config, text.as_bytes(), None, |_| None)
            .unwrap();

        let size = events.size_hint();
        let mut ranges = Vec::with_capacity(size.1.unwrap_or(size.0) / 3);
        let (mut hl, mut st, mut en) = ("", 0, 0);

        use HighlightEvent::*;
        while let Some(Ok(event)) = events.next() {
            // TODO: this is wrong, you could have a final `HighlightStart`
            // event w/o a following `Source`.
            match event {
                HighlightStart(Highlight(i)) => hl = TS_HLGROUPS[i],
                Source { start, end } => (st, en) = (start, end),
                HighlightEnd => ranges.push((st..en, hl)),
            }
        }

        ranges
    }
}

// TODO: add tests!
