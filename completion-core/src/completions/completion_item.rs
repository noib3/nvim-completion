use std::borrow::Cow;
use std::ops::RangeInclusive;

use crate::hlgroups;

/// TODO: docs
#[derive(Debug, Clone, Default)]
pub struct CompletionItem {
    pub(crate) text: String,

    icon: Option<char>,

    /// Used to color the completion item when shown inside the completion
    /// menu's buffer.
    ///
    /// Each item in the vector is a `(byte_range, highlight_group)` tuple,
    /// where `highlight_group` is the name of a Neovim highlight group and
    /// `byte_range` is a range of bytes of the [`text`] string.
    pub(crate) highlight_ranges: Vec<(RangeInclusive<usize>, &'static str)>,
}

impl CompletionItem {
    #[inline]
    pub(crate) fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn single_line_display(&self) -> Cow<'_, str> {
        Cow::Owned(format!(
            " {} ",
            crate::utils::single_line_display(&self.text)
        ))
    }

    /// TODO: docs
    pub(crate) fn set_matched_bytes<I>(&mut self, bytes: I)
    where
        I: IntoIterator<Item = RangeInclusive<usize>>,
    {
        let prefix_offset = 1 + match self.icon {
            Some(ch) => ch.len_utf8() + 1,
            None => 0,
        };

        let bytes = bytes.into_iter().map(|range| {
            let (mut start, mut end) = range.into_inner();
            start += prefix_offset;
            end += prefix_offset;
            (RangeInclusive::new(start, end), hlgroups::MENU_MATCHING)
        });

        self.highlight_ranges.extend(bytes);
    }

    /// TODO: docs
    pub fn highlight_icon(&mut self, hl_group: &'static str) {
        debug_assert!(self.icon.is_some());
        let icon_len = self.icon.as_ref().unwrap().len_utf8();
        self.highlight_ranges.push((1..=1 + icon_len, hl_group));
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn matched_text(&self) -> &str {
        &self.text
    }
}

/// TODO: docs
#[derive(Debug, Clone)]
pub struct CompletionItemBuilder {
    text: Option<String>,
}

impl CompletionItemBuilder {
    /// TODO: docs
    #[inline(always)]
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: Some(text.into()) }
    }

    /// TODO: docs
    #[inline(always)]
    pub fn build(mut self) -> CompletionItem {
        CompletionItem {
            text: self.text.take().unwrap(),
            ..Default::default()
        }
    }
}
