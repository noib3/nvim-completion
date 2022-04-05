use std::collections::HashMap;
use std::fmt::Display;

use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;
use crate::constants::*;

/// Echoes an error message.
pub fn echoerr<M: Display>(lua: &Lua, msg: M) -> LuaResult<()> {
    self::echo(lua, msg.to_string(), HLGROUP_ERROR_MSG_TAG)
}

/// Echoes a warning message.
pub fn echowar<M: Display>(lua: &Lua, msg: M) -> LuaResult<()> {
    self::echo(lua, msg.to_string(), HLGROUP_WARNING_MSG_TAG)
}

// TODO: create highlight group
/// Echoes a info message.
pub fn echoinfo<M: Display>(lua: &Lua, msg: M) -> LuaResult<()> {
    self::echo(lua, msg.to_string(), HLGROUP_WARNING_MSG_TAG)
}

/// Echoes a nicely highlighted message and adds it to the message history.
fn echo(lua: &Lua, msg: String, hl_group_tag: &'static str) -> LuaResult<()> {
    let msg_chunks = to_highlighted_chunks(
        msg,
        vec![(b'`', HLGROUP_OPTION_PATH), (b'"', HLGROUP_MSG_FIELD)],
    );

    let chunks = [
        vec![(MSG_TAG.into(), Some(hl_group_tag)), (" ".into(), None)],
        msg_chunks,
    ]
    .concat();

    api::echo(lua, chunks, true)
}

/// Takes a string and a vector of `(separator, hl_group)` tuples, returns a
/// vector with elements of the form `(text, Option<hl_group>)`.
/// Pretty simplistic but gets the job done.
fn to_highlighted_chunks(
    msg: String,
    separators: Vec<(u8, &'static str)>,
) -> Vec<(String, Option<&'static str>)> {
    let map = separators
        .into_iter()
        .collect::<HashMap<u8, &'static str>>();

    let mut start = 0;
    let mut current_hl = None;
    let mut chunks = Vec::new();

    for (i, byte) in msg.bytes().enumerate() {
        // Interesting things only happen when the current byte is a separator.
        if let Some(&hl_group) = map.get(&byte) {
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
    use super::to_highlighted_chunks;

    #[test]
    fn no_sep() {
        assert_eq!(
            vec![("foo".into(), None)],
            to_highlighted_chunks("foo".into(), vec![])
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
            to_highlighted_chunks("'foo'".into(), vec![(b'\'', "Bar")])
        );
    }

    #[test]
    fn not_closed() {
        assert_eq!(
            vec![("'".into(), None), ("foo".into(), None)],
            to_highlighted_chunks("'foo".into(), vec![(b'\'', "Bar")])
        );
    }

    #[test]
    fn dangling() {
        assert_eq!(
            vec![("foo'".into(), None)],
            to_highlighted_chunks("foo'".into(), vec![(b'\'', "Bar")])
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
            to_highlighted_chunks(
                "this 'is' a `test`".into(),
                vec![(b'\'', "Foo"), (b'`', "Bar")]
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
            to_highlighted_chunks(
                "this 'is `a` test'".into(),
                vec![(b'\'', "Foo"), (b'`', "Bar")]
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
            to_highlighted_chunks(
                "this 'is `a' test`".into(),
                vec![(b'\'', "Foo"), (b'`', "Bar")]
            )
        );
    }
}
