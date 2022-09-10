use std::borrow::Cow;
use std::ops::RangeInclusive;

/// TODO: docs
pub(crate) fn single_line_display(text: &str) -> Cow<'_, str> {
    match memchr::memchr(b'\n', text.as_bytes()) {
        Some(idx) => Cow::Owned(format!("{}..", &text[..idx])),
        None => Cow::Borrowed(text),
    }
}

/// TODO: docs
pub(crate) fn to_ranges(v: &[usize]) -> Vec<RangeInclusive<usize>> {
    if v.is_empty() {
        return vec![];
    }

    let mut ranges = Vec::new();

    let mut start = v[0];
    let mut current = start;

    for &char_idx in v.iter().skip(1) {
        if char_idx != current + 1 {
            ranges.push(start..=current + 1);
            start = char_idx;
            current = start;
        }

        current += 1;
    }

    let last = v.last().unwrap();

    if ranges.is_empty() || *ranges.last().unwrap().end() != last + 1 {
        ranges.push(start..=last + 1);
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ranges() {
        let v = [];
        assert!(to_ranges(&v).is_empty());

        let v = [0, 1, 2];
        assert_eq!(&[0..=3], &*to_ranges(&v));

        let v = [0, 1, 3];
        assert_eq!(&[0..=2, 3..=4], &*to_ranges(&v));

        let v = [0, 2];
        assert_eq!(&[0..=1, 2..=3], &*to_ranges(&v));
    }

    #[test]
    fn display_singleline() {
        assert_eq!("foo", single_line_display("foo"));
    }

    #[test]
    fn display_multiline() {
        assert_eq!("foo..", single_line_display("foo\nbar"));
    }

    #[test]
    fn display_trailing_newline() {
        assert_eq!("foo..", single_line_display("foo\n"));
    }

    #[test]
    fn display_trailing_bra() {
        assert_eq!("foo{", single_line_display("foo{"));
    }

    // #[test]
    // fn display_multiline_brace() {
    //     assert_eq!("foo{(..)}", single_line_display("foo{(\n"));
    //     assert_eq!("foo{bar(..)}", single_line_display("foo{bar(\n"));
    //     assert_eq!("foo(bar{..})", single_line_display("foo(bar{\n"));
    //     assert_eq!("foo)bar}..", single_line_display("foo)bar}\n"));
    //     assert_eq!("foo)bar}..", single_line_display("foo)bar}{(\n"));
    // }
}
