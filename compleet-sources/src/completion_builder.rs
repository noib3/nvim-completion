use super::completion_item::{
    CompletionItem,
    Details,
    HighlightRange,
    PostInsertCallback,
};

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
