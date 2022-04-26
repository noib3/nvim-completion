use std::ops::Range;

use mlua::prelude::{Lua, LuaResult};
use unicode_segmentation::UnicodeSegmentation;

// TODO: make this more similar to
// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem.

pub type Completions = Vec<CompletionItem>;

pub type PostInsertCallback = Box<
    dyn 'static
        + Send
        + for<'callback> Fn(&'callback Lua, CompletionItem) -> LuaResult<()>,
>;

#[derive(Default)]
pub struct CompletionItem {
    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,

    /// An icon representing the type of completion.
    pub icon: Option<String>,

    /// The text shown in the details window as a vector of strings.
    pub details: Option<Vec<String>>,

    /// A callback to be executed **after** the completion has been inserted
    /// into the buffer.
    pub post_insert_callback: Option<PostInsertCallback>,

    /// A vector of `(hl_group, byte_range)` tuples where all bytes in
    /// `byte_range` are highlighted in the `hl_group` highlight group.
    hl_ranges: Vec<(&'static str, Range<usize>)>,

    /// The formatted completion item as shown in the completion menu.
    format: Option<String>,

    /// The number of the extended grapheme clusters in the formatted
    /// completion item (i.e. a more accurate notion of "string length" than
    /// both `string.len()` and `string.chars().count()`).
    len: Option<usize>,
}

impl CompletionItem {
    /// The number of bytes before the start of the item label in the formatted
    /// string.
    pub fn label_byte_offset(&self) -> usize {
        match &self.icon {
            Some(icon) => 2 + icon.len(),
            None => 1,
        }
    }

    pub fn hl_ranges(&self) -> &'_ [(&'static str, Range<usize>)] {
        &self.hl_ranges
    }

    // We take a `&mut self` to memoize the result.
    pub fn format(&mut self) -> String {
        self.format.clone().unwrap_or({
            let fmt = match &self.icon {
                Some(icon) => format!(" {icon} {} ", self.text),
                None => format!(" {} ", self.text),
            };
            self.format = Some(fmt.clone());
            fmt
        })
    }

    // We take a `&mut self` to memoize the result.
    pub fn len(&mut self) -> usize {
        self.len.unwrap_or({
            let len = self.format().graphemes(true).count();
            self.len = Some(len);
            len
        })
    }
}

#[derive(Default)]
pub struct CompletionItemBuilder {
    text: String,
    icon: Option<String>,
    details: Option<String>,
    post_insert_callback: Option<PostInsertCallback>,
    hl_ranges: Vec<(&'static str, Range<usize>)>,
}

impl CompletionItemBuilder {
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self { text: text.as_ref().to_string(), ..Default::default() }
    }

    pub fn icon(mut self, icon: char, hl_group: Option<&'static str>) -> Self {
        let icon = icon.to_string();

        if let Some(hl_group) = hl_group {
            self.hl_ranges.push((hl_group, (1..1 + icon.len())))
        }

        self.icon = Some(icon);
        self
    }

    pub fn details<D: AsRef<str>>(mut self, details: D) -> Self {
        self.details = Some(details.as_ref().to_string());
        self
    }

    pub fn _post_insert_callback(mut self, cb: PostInsertCallback) -> Self {
        self.post_insert_callback = Some(cb);
        self
    }

    pub fn build(self) -> CompletionItem {
        let CompletionItemBuilder {
            text,
            icon,
            details,
            post_insert_callback,
            hl_ranges,
        } = self;

        let details = details.map(|str| {
            str.split("\n").map(|line| line.into()).collect::<Vec<String>>()
        });

        CompletionItem {
            text,
            icon,
            details,
            post_insert_callback,
            hl_ranges,
            ..Default::default()
        }
    }
}
