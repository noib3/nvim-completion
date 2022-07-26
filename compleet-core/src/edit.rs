use std::borrow::Cow;
use std::ops::RangeInclusive;

use nvim_oxi::{self as nvim, opts::OnBytesArgs};
use ropey::Rope;

/// An edit applied to an attached buffer.
pub(crate) enum Edit<'ins> {
    Insertion(RangeInclusive<usize>, Cow<'ins, str>),
    Deletion(RangeInclusive<usize>),
}

impl<'ins> Edit<'ins> {
    pub(crate) fn apply_to_rope(&self, rope: &mut Rope) {
        match self {
            Edit::Insertion(range, text) => {
                let start = rope.byte_to_char(*range.start());
                let end = rope.byte_to_char(*range.end());
                if start < end {
                    rope.remove(start..=end);
                }
                rope.insert(start, text);
            },

            Edit::Deletion(range) => {
                let start = rope.byte_to_char(*range.start());
                let end = rope.byte_to_char(*range.end());
                rope.remove(start..=end);
            },
        }
    }
}

impl<'ins> TryFrom<&OnBytesArgs> for Edit<'ins> {
    type Error = nvim::Error;

    fn try_from(args: &OnBytesArgs) -> Result<Self, Self::Error> {
        let buf = &args.1;
        todo!()
    }
}
