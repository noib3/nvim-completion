use std::ops::Range;

use mlua::prelude::{Lua, LuaResult};
use tree_sitter_highlight::Highlighter;
use unicode_segmentation::UnicodeSegmentation;

use crate::treesitter::TSConfig;

// TODO: make this more similar to
// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem.

pub type Completions = Vec<CompletionItem>;

pub(crate) type HlRange = (Range<usize>, &'static str);

pub(crate) type PostInsertCallback = Box<
    dyn 'static
        + Send
        + for<'callback> Fn(&'callback Lua, CompletionItem) -> LuaResult<()>,
>;

#[derive(Default)]
pub struct CompletionItem {
    /// An icon representing the type of completion.
    pub icon: Option<String>,

    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,

    /// TODO: docs
    pub infos: Option<String>,

    /// TODO: docs
    pub details: Option<Details>,

    /// A callback to be executed **after** the completion has been inserted
    /// into the buffer.
    pub post_insert_callback: Option<PostInsertCallback>,

    /// A vector of `(byte_range, hl_group)` tuples where all bytes in
    /// `byte_range` will be highlighted with the `hl_group` highlight group.
    hl_ranges: Vec<HlRange>,

    /// The formatted completion item as shown in the completion menu.
    format: Option<String>,

    /// The number of the extended grapheme clusters in the formatted
    /// completion item (i.e. a more accurate notion of "string length" than
    /// both `string.len()` and `string.chars().count()`).
    len: Option<usize>,
}

#[derive(Default)]
pub struct Details {
    /// The text shown in the details window as a vector of strings.
    pub text: Vec<String>,

    /// The filetype of the buffer used to display the text.
    pub filetype: String,
}

impl<T: Into<String>> From<T> for CompletionItem {
    fn from(t: T) -> Self {
        Self { text: t.into(), ..Default::default() }
    }
}

impl CompletionItem {
    /// Returns the number of bytes before the start of the completion's text
    /// in the formatted string.
    pub fn text_byte_offset(&self) -> usize {
        // If the completion has an icon the offset is the icon's length
        // plus 2 for its leading and following space, if if doesn't it's just
        // 1 for the leading space.
        self.icon.as_ref().map(|icon| 2 + icon.len()).unwrap_or(1)
    }

    /// TODO: docs
    pub fn hl_ranges(&self) -> &'_ [HlRange] {
        &self.hl_ranges
    }

    /// Returns the formatted string displayed in the completion menu.
    pub fn format(&mut self) -> String {
        let format = self.format.get_or_insert(format!(
            "{} {} {}",
            self.icon.as_ref().map_or_else(String::new, |v| format!(" {v}")),
            self.text,
            self.infos.as_ref().map_or_else(String::new, |v| format!("{v} ")),
        ));

        format.clone()
    }

    /// TODO: docs
    pub fn highlight_icon(&mut self, hl_group: &'static str) {
        if let Some(icon) = &self.icon {
            self.hl_ranges.push((1..1 + icon.len(), hl_group))
        }
    }

    /// TODO: docs
    pub fn highlight_text<R>(&mut self, ranges: R)
    where
        R: IntoIterator<Item = HlRange>,
    {
        let offset = self.text_byte_offset();
        for (Range { start, end }, group) in ranges {
            debug_assert!(end <= self.text.len());
            self.hl_ranges.push(((offset + start)..(offset + end), group));
        }
    }

    /// Highlights the completion's text using a treesitter config.
    pub fn ts_highlight_text(
        &mut self,
        highlighter: &mut Highlighter,
        config: &TSConfig,
    ) {
        self.highlight_text(config.highlight(highlighter, &self.text))
    }

    /// The length in grapheme clusters of the completion's formatted string.
    pub fn len(&mut self) -> usize {
        let len = self.format().graphemes(true).count();
        *self.len.get_or_insert(len)
    }

    /// TODO: docs
    pub fn set_details<S: Into<String>>(&mut self, details: S, ft: S) {
        let text = details
            .into()
            .split("\n")
            .map(|line| line.into())
            .collect::<Vec<String>>();

        self.details =
            Some(Details { text, filetype: ft.into(), ..Default::default() });
    }
}
