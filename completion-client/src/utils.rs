use std::borrow::Cow;

/// TODO: docs
pub(crate) fn single_line_display(text: &str) -> Cow<'_, str> {
    match memchr::memchr(b'\n', text.as_bytes()) {
        Some(idx) => Cow::Owned(format!("{}..", &text[..idx])),
        None => Cow::Borrowed(text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
