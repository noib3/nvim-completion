use std::borrow::Cow;

use completion_types::{CompletionItem, Position};
use nvim::api::{
    self,
    opts::SetExtmarkOpts,
    types::ExtmarkVirtTextPosition,
    Buffer,
};
use nvim_oxi as nvim;
use serde::Deserialize;

use crate::hlgroups;
use crate::PositionExt;

const HINT_NAMESPACE: &str = "completion_hint";

#[derive(Debug)]
pub(crate) struct CompletionHint {
    config: HintConfig,

    /// TODO: docs
    namespace_id: u32,

    /// TODO: docs
    extmark_id: Option<u32>,

    /// TODO: docs
    opts: SetExtmarkOpts,
}

impl Default for CompletionHint {
    #[inline]
    fn default() -> Self {
        let namespace_id = api::create_namespace(HINT_NAMESPACE);

        let opts = SetExtmarkOpts::builder()
            .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
            .build();

        Self {
            namespace_id,
            opts,
            extmark_id: None,
            config: HintConfig::default(),
        }
    }
}

impl CompletionHint {
    #[inline]
    pub(super) fn init(&mut self, config: HintConfig) {
        self.config = config;
    }

    #[inline]
    pub fn is_visible(&self) -> bool {
        self.extmark_id.is_some()
    }

    /// Hides the completion hint in the buffer.
    pub fn hide(&mut self, buf: &mut Buffer) -> nvim::Result<()> {
        buf.clear_namespace(self.namespace_id, 0, usize::MAX)?;
        self.extmark_id = None;
        Ok(())
    }

    /// Shows the completion hint in the provided buffer.
    pub(super) fn show(
        &mut self,
        completion: &CompletionItem,
        buf: &mut Buffer,
        cursor: &Position,
    ) -> nvim::Result<()> {
        if !self.config.enable {
            return Ok(());
        }

        let text = match extract_hint_text(cursor, completion) {
            Some(text) => text,

            None => {
                if self.is_visible() {
                    self.hide(buf)?;
                }
                return Ok(());
            },
        };

        self.opts.set_id(self.extmark_id.unwrap_or(1));
        self.opts.set_virt_text([(text, [hlgroups::HINT])]);

        self.extmark_id = Some(buf.set_extmark(
            self.namespace_id,
            cursor.row as usize,
            cursor.col as usize,
            &self.opts,
        )?);

        Ok(())
    }
}

/// TODO: docs
fn extract_hint_text<'a>(
    cursor: &'a Position,
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
    // NOTE: it might make sense to relax this constrait in the future. For
    // example, if the current line is `foo.bar|` and the completion is `baz`
    // we could overlay the `z` on top of the `r` for a better preview
    // experience.
    //
    if !cursor.is_at_eol()
        || !completion.text.starts_with(cursor.matched_prefix())
    {
        return None;
    }

    Some(crate::utils::single_line_display(
        &completion.text[cursor.len_prefix()..],
    ))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HintConfig {
    #[serde(default = "yes")]
    enable: bool,
}

impl Default for HintConfig {
    fn default() -> Self {
        HintConfig { enable: yes() }
    }
}

fn yes() -> bool {
    true
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn cursor_not_at_eol() {
//         let cursor = LineContext::new(0, 2, "foo".into());
//         let comp = CompletionItem::new("foobar");
//         assert_eq!(None, extract_hint_text(&cursor, &comp));
//     }

//     #[test]
//     fn foo_foobar() {
//         let cursor = LineContext::new(0, 3, "foo".into());
//         let comp = CompletionItem::new("foobar");
//         assert_eq!("bar", extract_hint_text(&cursor, &comp).unwrap());
//     }

//     #[test]
//     fn failing() {
//         let cursor = LineContext::new(0, 1, "e".into());
//         let comp = CompletionItem::new("lsp received a");
//         assert_eq!(
//             "sp received a",
//             extract_hint_text(&cursor, &comp).unwrap()
//         );
//     }

//     #[test]
//     fn multiline_completion() {
//         let cursor = LineContext::new(0, 3, "foo".into());
//         let comp = CompletionItem::new("foobar\nbaz");
//         assert_eq!("bar..", extract_hint_text(&cursor, &comp).unwrap());
//     }

//     #[test]
//     fn multiword_completion() {
//         let cursor = LineContext::new(0, 4, "aaaa".into());
//         let comp = CompletionItem::new("lsp received a\nbaz");
//         assert_eq!("received a..", extract_hint_text(&cursor, &comp).unwrap());
//     }

//     #[test]
//     fn prefix_longer_than_completion() {
//         let cursor = LineContext::new(0, 11, "foo.bar_baz".into());
//         let comp = CompletionItem::new("bar");
//         assert_eq!(None, extract_hint_text(&cursor, &comp));
//     }
// }
