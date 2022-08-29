use std::borrow::Cow;

use nvim_oxi::{
    self as nvim,
    api::{self, Buffer},
    opts::SetExtmarkOpts,
    types::ExtmarkVirtTextPosition,
};

use crate::completions::Cursor;
use crate::hlgroups;
use crate::CompletionItem;

const HINT_NAMESPACE: &str = "completion_hint";

pub(crate) struct CompletionHint {
    namespace_id: u32,
    extmark_id: Option<u32>,
    opts: SetExtmarkOpts,
}

impl Default for CompletionHint {
    #[inline]
    fn default() -> Self {
        let namespace_id = api::create_namespace(HINT_NAMESPACE);

        let opts = SetExtmarkOpts::builder()
            .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
            .build();

        Self { namespace_id, opts, extmark_id: None }
    }
}

impl CompletionHint {
    pub fn is_visible(&self) -> bool {
        self.extmark_id.is_some()
    }

    /// Hides the completion hint from the providec buffer.
    pub fn hide(&mut self, buf: &mut Buffer) -> nvim::Result<()> {
        buf.clear_namespace(self.namespace_id, 0, usize::MAX)?;
        self.extmark_id = None;
        Ok(())
    }

    /// Shows the completion hint in the provided buffer.
    pub fn show(
        &mut self,
        buf: &mut Buffer,
        cursor: &Cursor,
        completion: &CompletionItem,
    ) -> nvim::Result<()> {
        let text = match extract_hint_text(cursor, completion) {
            Some(text) => text,
            None => return Ok(()),
        };

        self.opts.set_id(self.extmark_id.unwrap_or(1));
        self.opts.set_virt_text([(text, [hlgroups::HINT])]);

        self.extmark_id = Some(buf.set_extmark(
            self.namespace_id,
            cursor.row,
            cursor.col,
            &self.opts,
        )?);

        Ok(())
    }
}

/// TODO: docs
fn extract_hint_text<'a>(
    cursor: &'a Cursor,
    completion: &'a CompletionItem,
) -> Option<Cow<'a, str>> {
    // We only display completion hints if there are no characters after the
    // cursor in the current line. Not doing so would cause either the hint to
    // overlay actual text or the text to be shifted to the right.
    //
    // Also, if the completion text is shorter than the current prefix we
    // return early. For example if the current line is `foo.barbaz|` and the
    // completion is `bar`.
    //
    // This is generally not the case because the completions should have
    // already been filtered by the time they're displayed, but we're not
    // making this assumption here.
    //
    // NOTE: it might make sense to relax this constrait in the future. For
    // example, if the current line is `foo.bar|` and the completion is `baz`
    // we could overlay the `z` on top of the `r` for a better preview
    // experience.
    if !cursor.is_at_eol() || completion.text.len() < cursor.len_prefix {
        return None;
    }

    Some(crate::utils::single_line_display(
        &completion.text[cursor.len_prefix..],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_not_at_eol() {
        let cursor = Cursor::new(0, 2, "foo".into());
        let comp = CompletionItem::new("foobar");
        assert_eq!(None, extract_hint_text(&cursor, &comp));
    }

    #[test]
    fn foo_foobar() {
        let cursor = Cursor::new(0, 3, "foo".into());
        let comp = CompletionItem::new("foobar");
        assert_eq!("bar", extract_hint_text(&cursor, &comp).unwrap());
    }

    #[test]
    fn failing() {
        let cursor = Cursor::new(0, 1, "e".into());
        let comp = CompletionItem::new("lsp received a");
        assert_eq!(
            "sp received a",
            extract_hint_text(&cursor, &comp).unwrap()
        );
    }

    #[test]
    fn multiline_completion() {
        let cursor = Cursor::new(0, 3, "foo".into());
        let comp = CompletionItem::new("foobar\nbaz");
        assert_eq!("bar..", extract_hint_text(&cursor, &comp).unwrap());
    }

    #[test]
    fn multiword_completion() {
        let cursor = Cursor::new(0, 4, "aaaa".into());
        let comp = CompletionItem::new("lsp received a\nbaz");
        assert_eq!("received a..", extract_hint_text(&cursor, &comp).unwrap());
    }

    #[test]
    fn prefix_longer_than_completion() {
        let cursor = Cursor::new(0, 11, "foo.bar_baz".into());
        let comp = CompletionItem::new("bar");
        assert_eq!(None, extract_hint_text(&cursor, &comp));
    }
}
