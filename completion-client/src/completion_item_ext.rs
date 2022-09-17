use std::borrow::Cow;
use std::ops::RangeInclusive;

use completion_types::CompletionItem;

pub(crate) trait CompletionItemExt {
    fn hint_display(&self) -> Cow<'_, str>;

    fn menu_display(&self) -> String;

    fn highlight_ranges(&self) -> Vec<(RangeInclusive<usize>, &'static str)>;
}

impl CompletionItemExt for CompletionItem {
    fn hint_display(&self) -> Cow<'_, str> {
        match memchr::memchr(b'\n', self.text.as_bytes()) {
            Some(idx) => Cow::Owned(format!("{}..", &self.text[..idx])),
            None => Cow::Borrowed(&self.text),
        }
    }

    fn menu_display(&self) -> String {
        format!(" {} ", self.hint_display())
    }

    fn highlight_ranges(&self) -> Vec<(RangeInclusive<usize>, &'static str)> {
        vec![]
    }
}
