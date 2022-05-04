use std::collections::HashMap;
use std::fmt::Display;

use bindings::{api, r#fn};
use mlua::prelude::{Lua, LuaResult};

use crate::constants::{hlgroups::messages, MSG_TAG};

/// Echoes an error message.
pub fn echoerr<M: Display>(lua: &Lua, msg: M) -> LuaResult<()> {
    self::echo(lua, msg.to_string(), messages::ERROR_MSG_TAG)
}

/// Echoes a warning message.
pub fn echowar<M: Display>(lua: &Lua, msg: M) -> LuaResult<()> {
    self::echo(lua, msg.to_string(), messages::WARNING_MSG_TAG)
}

// TODO: create highlight group
/// Echoes an info message.
pub fn echoinfo<M: Display>(lua: &Lua, msg: M) -> LuaResult<()> {
    self::echo(lua, msg.to_string(), messages::INFO_MSG_TAG)
}

fn echo(lua: &Lua, msg: String, hl_group_tag: &'static str) -> LuaResult<()> {
    let msg_chunks = to_highlighted_chunks(
        msg,
        vec![('`', messages::OPTION_PATH), ('"', messages::MSG_FIELD)],
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
    separators: Vec<(char, &'static str)>,
) -> Vec<(String, Option<&'static str>)> {
    let map = separators.into_iter().collect::<HashMap<char, &'static str>>();

    let mut start = 0;
    let mut current_hl = None;
    let mut chunks = Vec::new();

    for (i, char) in msg.chars().enumerate() {
        // Interesting things only happen when the current character is a separator.
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

pub fn get_screen_cursor(lua: &Lua) -> LuaResult<(u16, u16)> {
    let (row, col) = api::win_get_cursor(lua, 0)?;

    let pos = r#fn::screenpos(lua, row, col + 1)?;

    Ok((pos.get::<_, u16>("row")?, pos.get::<_, u16>("col")? - 1))
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
            to_highlighted_chunks("'foo'".into(), vec![('\'', "Bar")])
        );
    }

    #[test]
    fn not_closed() {
        assert_eq!(
            vec![("'".into(), None), ("foo".into(), None)],
            to_highlighted_chunks("'foo".into(), vec![('\'', "Bar")])
        );
    }

    #[test]
    fn dangling() {
        assert_eq!(
            vec![("foo'".into(), None)],
            to_highlighted_chunks("foo'".into(), vec![('\'', "Bar")])
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
            to_highlighted_chunks(
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
            to_highlighted_chunks(
                "this 'is `a' test`".into(),
                vec![('\'', "Foo"), ('`', "Bar")]
            )
        );
    }
}
