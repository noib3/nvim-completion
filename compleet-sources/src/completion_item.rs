use mlua::Lua;
use unicode_segmentation::UnicodeSegmentation;

/// A range used to highlight all bytes of the formatted completion label in
/// the `bytes` range with the `group` highlight group.
#[derive(Debug)]
pub struct HighlightRange {
    pub bytes: std::ops::Range<usize>,
    pub group: &'static str,
}

/// A callback that's executed **after** a completion has been selected and
/// inserted into the buffer.
pub type PostInsertCallback = Box<
    dyn 'static
        + Send
        + for<'callback> FnMut(
            &'callback Lua,
            CompletionItem,
        ) -> mlua::Result<()>,
>;

/// Additional information about a completion item that's displayed in a
/// separate floating window when the item is selected.
#[derive(Debug, Default)]
pub struct Details {
    pub text: Vec<String>,
    pub ft: String,
}

pub type Completions = Vec<CompletionItem>;

/// The main point of the whole plugin.
#[derive(Default)]
pub struct CompletionItem {
    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,

    /// An icon representing the type of completion.
    pub icon: Option<char>,

    /// The text that's shown in the completion menu. It defaults to the first
    /// line of the `text` field if it's not set. Should **not** contain any
    /// newlines.
    pub label: String,

    /// Additional informations about the item, also shown in the completion
    /// menu and ragged left (meaning the last character of the string will
    /// touch the right edge of the completion menu). Should only be used for
    /// short infos, a few words at most.
    pub infos: Option<String>,

    /// Additional information about a completion item that's displayed in a
    /// separate floating window when the item is selected. Can be used for
    /// longer metadata like documentation.
    pub details: Option<Details>,

    /// A callback that's executed **after** a completion has been selected
    /// and inserted into the buffer.
    pub post_insert_callback: Option<PostInsertCallback>,

    /// The text shown in the completion menu to represent this item. Made
    /// from the `icon`, the `label` and the `infos`.
    format: Option<String>,

    highlight_ranges: Vec<HighlightRange>,

    /// The number of the extended grapheme clusters in the formatted
    /// completion item (i.e. a more accurate notion of "string length" than
    /// both `string.len()` and `string.chars().count()`).
    _len: Option<usize>,
}

impl CompletionItem {
    /// Returns the number of bytes before the start of the completion's label
    /// in the formatted string.
    pub fn label_byte_offset(&self) -> usize {
        // If the completion has an icon the offset is the icon's length
        // plus 2 for its leading and following space, if if doesn't it's just
        // 1 for the leading space.
        self.icon.as_ref().map(|icon| 2 + icon.len_utf8()).unwrap_or(1)
    }

    /// Returns an iterator over the highlight ranges of the completion.
    pub fn highlight_ranges(&self) -> &[HighlightRange] {
        &self.highlight_ranges
    }

    /// Returns the formatted string displayed in the completion menu.
    pub fn format(&mut self) -> &str {
        self.format.get_or_insert(format!(
            "{} {} {}",
            self.icon.as_ref().map_or_else(String::new, |i| format!(" {i}")),
            self.label,
            self.infos.as_ref().map_or_else(String::new, |i| format!("{i} ")),
        ))
    }

    /// The length in grapheme clusters of the completion's formatted string.
    pub fn len(&mut self) -> usize {
        // *self.len.get_or_insert(self.format().graphemes(true).count())
        self.format().graphemes(true).count()
    }

    /// Sets the highlight group of the completion item's icon if the icon is
    /// already set, does nothing otherwise.
    pub fn highlight_icon(&mut self, group: &'static str) {
        if let Some(icon) = &self.icon {
            self.highlight_ranges.push(HighlightRange {
                bytes: (1..1 + icon.len_utf8()),
                group,
            })
        }
    }

    /// TODO: docs
    pub fn highlight_label<Ranges>(&mut self, ranges: Ranges)
    where
        Ranges: IntoIterator<Item = (std::ops::Range<usize>, &'static str)>,
    {
        let offset = self.label_byte_offset();

        self.highlight_ranges.extend(ranges.into_iter().map(
            |(bytes, group)| HighlightRange {
                bytes: (offset + bytes.start)..(offset + bytes.end),
                group,
            },
        ))
    }
}

#[derive(Default)]
pub struct CompletionItemBuilder {
    text: Option<String>,
    icon: Option<char>,
    label: Option<String>,
    infos: Option<String>,
    details_text: Option<Vec<String>>,
    details_ft: Option<String>,
    post_insert_callback: Option<PostInsertCallback>,
    highlight_ranges: Option<Vec<HighlightRange>>,
}

impl CompletionItemBuilder {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self { text: Some(text.into()), ..Default::default() }
    }

    pub fn icon(&mut self, icon: char) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn details_text<S: Into<String>>(&mut self, text: S) -> &mut Self {
        self.details_text = Some(
            text.into()
                .lines()
                .map(|line| line.to_owned())
                .collect::<Vec<String>>(),
        );
        self
    }

    pub fn details_ft<S: Into<String>>(&mut self, ft: S) -> &mut Self {
        self.details_ft = Some(ft.into());
        self
    }

    pub fn build(&mut self) -> CompletionItem {
        let text = self.text.take().unwrap();

        // TODO: come up with a better logic for this. For example, if the text
        // is:
        // ```txt
        // foo {
        //   bar
        // }
        // ```
        // then the `label` would be set to `foo {`. A better label would be
        // `foo {..}`. Look into how treesitter sets the visible line in folded
        // text.
        let label = self.label.take().unwrap_or(
            text.lines().next().map(|s| s.to_owned()).unwrap_or_default(),
        );

        let details = self.details_text.take().map(|text| Details {
            text,
            ft: self.details_ft.take().unwrap_or_default(),
        });

        CompletionItem {
            text,
            label,
            details,
            icon: self.icon.take(),
            infos: self.infos.take(),
            post_insert_callback: self.post_insert_callback.take(),
            highlight_ranges: self.highlight_ranges.take().unwrap_or_default(),
            ..Default::default()
        }
    }
}
