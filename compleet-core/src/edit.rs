use std::borrow::Cow;
use std::ops::RangeInclusive;

use nvim_oxi::{self as nvim, opts::OnBytesArgs};
use ropey::Rope;

/// An edit applied to an attached buffer.
pub(crate) struct Edit<'ins>(RangeInclusive<usize>, Option<Cow<'ins, str>>);

impl<'ins> Edit<'ins> {
    pub(crate) fn apply_to_rope(&self, rope: &mut Rope) {
        let (range, text) = (&self.0, &self.1);

        let start = rope.byte_to_char(*range.start());
        let end = rope.byte_to_char(*range.end());

        if start < end {
            rope.remove(start..=end);
        }

        if let Some(text) = text {
            rope.insert(start, &text)
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
