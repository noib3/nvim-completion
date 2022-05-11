use std::collections::HashMap;

use bindings::api;
use mlua::Lua;

use super::hlgroups;

/// The tag used as a prefix in all the messages.
const MSG_TAG: &str = "[nvim-compleet]";

#[macro_export]
macro_rules! echoerr {
    ($lua:tt, $($arg:tt)*) => {{
        use crate::messages::{echoes::echo, hlgroups};
        let msg = std::fmt::format(format_args!($($arg)*));
        echo($lua, msg, hlgroups::TAG_ERROR)
    }}
}

#[macro_export]
macro_rules! echowarn {
    ($lua:tt, $($arg:tt)*) => {{
        use crate::messages::{echoes::echo, hlgroups};
        let msg = std::fmt::format(format_args!($($arg)*));
        echo($lua, msg, hlgroups::TAG_WARNING)
    }}
}

#[macro_export]
macro_rules! echoinfo {
    ($lua:tt, $($arg:tt)*) => {{
        use crate::messages::{echoes::echo, hlgroups};
        let msg = std::fmt::format(format_args!($($arg)*));
        echo($lua, msg, hlgroups::TAG_INFOS)
    }}
}

pub use echoerr;
pub use echoinfo;
pub use echowarn;

pub fn echo(
    lua: &Lua,
    msg: String,
    tag_hlgroup: &'static str,
) -> mlua::Result<()> {
    let hls =
        vec![('`', hlgroups::BAD_CONFIG_PATH), ('"', hlgroups::MSG_FIELD)];

    let chunks = [
        vec![(MSG_TAG.into(), Some(tag_hlgroup)), (" ".into(), None)],
        into_highlighted_chunks(msg, hls),
    ]
    .concat();

    api::echo(lua, chunks, true)
}

/// Takes a string and a vector of `(separator, hl_group)` tuples, returns a
/// vector with elements of the form `(text, Option<hl_group>)`. Pretty
/// simplistic but gets the job done.
fn into_highlighted_chunks(
    msg: String,
    separators: Vec<(char, &'static str)>,
) -> Vec<(String, Option<&'static str>)> {
    let map = separators.into_iter().collect::<HashMap<char, &'static str>>();

    let mut start = 0;
    let mut current_hl = None;
    let mut chunks = Vec::new();

    for (i, char) in msg.chars().enumerate() {
        // Interesting things only happen when the current character is a
        // separator.
        if let Some(&hl_group) = map.get(&char) {
            match current_hl {
                None => {
                    chunks.push((msg[start..=i].into(), None));
                    start = i + 1;
                    current_hl = Some(hl_group);
                },

                Some(hl) if hl == hl_group => {
                    chunks.push((msg[start..i].into(), Some(hl)));
                    start = i;
                    current_hl = None;
                },

                _ => {},
            };
        }
    }

    // Unclosed chunks are re-added without a highlight group.
    if start < msg.len() {
        chunks.push((msg[start..].into(), None));
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::into_highlighted_chunks;

    #[test]
    fn no_sep() {
        assert_eq!(
            vec![("foo".into(), None)],
            into_highlighted_chunks("foo".into(), vec![])
        );
    }

    #[test]
    fn one_big() {
        assert_eq!(
            vec![
                ("'".into(), None),
                ("foo".into(), Some("Bar")),
                ("'".into(), None),
            ],
            into_highlighted_chunks("'foo'".into(), vec![('\'', "Bar")])
        );
    }

    #[test]
    fn not_closed() {
        assert_eq!(
            vec![("'".into(), None), ("foo".into(), None)],
            into_highlighted_chunks("'foo".into(), vec![('\'', "Bar")])
        );
    }

    #[test]
    fn dangling() {
        assert_eq!(
            vec![("foo'".into(), None)],
            into_highlighted_chunks("foo'".into(), vec![('\'', "Bar")])
        );
    }

    #[test]
    fn two_seps() {
        assert_eq!(
            vec![
                ("this '".into(), None),
                ("is".into(), Some("Foo")),
                ("' a `".into(), None),
                ("test".into(), Some("Bar")),
                ("`".into(), None),
            ],
            into_highlighted_chunks(
                "this 'is' a `test`".into(),
                vec![('\'', "Foo"), ('`', "Bar")]
            )
        );
    }

    #[test]
    fn nested() {
        assert_eq!(
            vec![
                ("this '".into(), None),
                ("is `a` test".into(), Some("Foo")),
                ("'".into(), None),
            ],
            into_highlighted_chunks(
                "this 'is `a` test'".into(),
                vec![('\'', "Foo"), ('`', "Bar")]
            )
        );
    }

    #[test]
    fn interleaved() {
        assert_eq!(
            vec![
                ("this '".into(), None),
                ("is `a".into(), Some("Foo")),
                ("' test`".into(), None),
            ],
            into_highlighted_chunks(
                "this 'is `a' test`".into(),
                vec![('\'', "Foo"), ('`', "Bar")]
            )
        );
    }
}
