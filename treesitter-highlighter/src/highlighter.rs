use std::fmt;

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

        let mut source_seen = false;
        let size = events.size_hint();
        let mut ranges = Vec::with_capacity(size.1.unwrap_or(size.0) / 3);
        let (mut hl, mut st, mut en) = ("", 0, 0);

        use HighlightEvent::*;
        while let Some(Ok(event)) = events.next() {
            // TODO: this is wrong, you could have a final `HighlightStart`
            // event w/o a following `Source`.
            match event {
                HighlightStart(Highlight(i)) => hl = TS_HLGROUPS[i],

                Source { start, end } => {
                    source_seen = true;
                    (st, en) = (start, end)
                },

                HighlightEnd => {
                    // Only add the current range if there was a preceding
                    // `Source` event.
                    if source_seen {
                        source_seen = false;
                        ranges.push((st..en, hl))
                    }
                },
            }
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::Highlighter;

    #[test]
    fn rust_mut() {
        let mut highlighter = Highlighter::from_filetype("rust").unwrap();

        let text = "&mut Foo";
        let groups = vec![
            (0..1, "TSOperator"),
            (1..4, "TSKeyword"),
            (5..8, "TSVariable"),
        ];

        assert_eq!(groups, highlighter.highlight(text))
    }

    #[test]
    fn rust_parenthesis() {
        let mut highlighter = Highlighter::from_filetype("rust").unwrap();

        let text = "self.foo (as Foo)";
        let groups = vec![
            (0..4, "TSVariableBuiltin"),
            (4..5, "TSPunctDelimiter"),
            (5..8, "TSField"),
            (9..10, "TSPunctBracket"),
            (10..12, "TSVariable"),
            (13..16, "TSVariable"),
            (16..17, "TSPunctBracket"),
        ];

        assert_eq!(groups, highlighter.highlight(text))
    }
}
