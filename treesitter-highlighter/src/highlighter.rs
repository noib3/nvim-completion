use std::{fmt, ops::Range};

use tree_sitter_highlight::{
    Highlight,
    HighlightConfiguration,
    HighlightEvent,
    Highlighter as OGHighlighter,
};

use crate::constants::{HIGHLIGHT_NAMES, TS_HLGROUPS};
use crate::generated::config_from_filetype;

pub struct Highlighter(HighlightConfiguration);

impl fmt::Debug for Highlighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Highlighter({:?})", self.0.language)
    }
}

impl Highlighter {
    pub fn from_filetype(ft: &str) -> Option<Self> {
        config_from_filetype(ft).map(|mut config| {
            config.configure(HIGHLIGHT_NAMES);
            Self(config)
        })
    }
}

impl Highlighter {
    /// TODO: docs
    pub fn highlight(
        &self,
        highlighter: &mut OGHighlighter,
        text: &str,
    ) -> Vec<(Range<usize>, &'static str)> {
        let mut events = highlighter
            .highlight(&self.0, text.as_bytes(), None, |_| None)
            .unwrap();

        let size = events.size_hint();
        let mut ranges = Vec::with_capacity(size.1.unwrap_or(size.0) / 3);
        let (mut hl, mut st, mut en) = ("", 0, 0);

        use HighlightEvent::*;
        while let Some(Ok(event)) = events.next() {
            match event {
                HighlightStart(Highlight(i)) => hl = TS_HLGROUPS[i],
                Source { start, end } => (st, en) = (start, end),
                HighlightEnd => ranges.push((st..en, hl)),
            }
        }

        ranges
    }
}
