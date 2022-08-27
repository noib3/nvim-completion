use std::borrow::Cow;

use nvim_oxi::{
    self as nvim,
    api::{self, Buffer},
    opts::SetExtmarkOpts,
    types::ExtmarkVirtTextPosition,
};

use crate::cursor::Cursor;
use crate::hlgroups;
use crate::CompletionItem;

const HINT_NAMESPACE: &str = "completion_hint";

pub(crate) struct CompletionHint {
    namespace_id: u32,
    extmark_id: Option<u32>,
    opts: Option<SetExtmarkOpts>,
}

impl Default for CompletionHint {
    #[inline]
    fn default() -> Self {
        let namespace_id = api::create_namespace(HINT_NAMESPACE);

        let opts = Some(
            SetExtmarkOpts::builder()
                .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
                .build(),
        );

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
        // to be removed, only for testing
        if self.is_visible() {
            return Ok(());
        }

        let text = match extract_hint_text(cursor, completion) {
            Some(text) => text,
            None => {
                return Ok(());
            },
        };

        // self.opts.set_id(self.extmark_id.unwrap_or(1));
        // self.opts.set_virt_text([(text, [hlgroups::HINT])]);

        // TODO: don't construct this every time we display a new hint. Build
        // it once when `CompletionHint` gets initialized and just modify its
        // text before calling `set_extmark`.
        let opts = SetExtmarkOpts::builder()
            .id(self.extmark_id.unwrap_or(1))
            .virt_text([(text, [hlgroups::HINT])])
            .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
            .build();

        self.extmark_id = Some(buf.set_extmark(
            self.namespace_id,
            cursor.row,
            cursor.col,
            Some(&opts),
        )?);

        nvim::print!("Sowing??");

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
    if !cursor.is_at_eol() {
        return None;
    }

    let text = &completion.text[cursor.len_prefix..];

    // TODO: we should probably check for Unicode points (i.e. `char`s) instead
    // of raw bytes.
    let text = match memchr::memchr(b'\n', text.as_bytes()) {
        Some(idx) => Cow::Owned(format!("{}..", &text[..idx])),
        None => Cow::Borrowed(text),
    };

    Some(text)
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
    fn multiline_completion() {
        let cursor = Cursor::new(0, 3, "foo".into());
        let comp = CompletionItem::new("foobar\nbaz");
        assert_eq!("bar..", extract_hint_text(&cursor, &comp).unwrap());
    }
}
