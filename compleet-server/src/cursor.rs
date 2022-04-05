use rmpv::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    /// The number of bytes between the start of the line and the cursor.
    pub bytes: u32,

    /// The text in the row the cursor is currently on.
    pub line: String,

    /// The row the cursor is currently on.
    pub row: u32,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            bytes: 0,
            line: "".into(),
            row: 0,
        }
    }
}

/// Encode a `Cursor` into a vector of msgpack values.
impl From<Cursor> for Vec<Value> {
    fn from(cursor: Cursor) -> Vec<Value> {
        vec![
            Value::from(cursor.bytes),
            Value::from(cursor.line),
            Value::from(cursor.row),
        ]
    }
}

/// Try to decode a `Cursor` from a vector of msgpack values.
impl TryFrom<Vec<Value>> for Cursor {
    type Error = &'static str;

    fn try_from(vec: Vec<Value>) -> Result<Cursor, Self::Error> {
        if vec.len() != 3 {
            return Err("cursor array should have 3 values");
        }

        let mut iter = vec.into_iter();

        let bytes = match iter.next() {
            Some(Value::Integer(n)) if n.is_u64() => {
                n.as_u64().expect("already checked that it's a u64") as u32
            },
            _ => return Err("bytes arent't valid"),
        };

        let line = match iter.next() {
            Some(Value::String(s)) if s.is_str() => {
                s.into_str().expect("already checked that it's valid utf8")
            },
            _ => return Err("line isn't valid"),
        };

        let row = match iter.next() {
            Some(Value::Integer(n)) if n.is_u64() => {
                n.as_u64().expect("already checked that it's a u64") as u32
            },
            _ => return Err("row isn't valid"),
        };

        Ok(Cursor { bytes, line, row })
    }
}

impl Cursor {
    /// Whether the cursor is at the end of the line.
    pub fn is_at_eol(&self) -> bool {
        self.bytes as usize == self.line.len()
    }

    /// Whether the cursor is at the start of the line.
    pub fn _is_at_sol(&self) -> bool {
        self.bytes == 0
    }

    /// The number of bytes between the cursor and the first whitespace
    /// character before it.
    fn non_whitespace_bytes_pre(&self) -> usize {
        self.line[..self.bytes as usize]
            .bytes()
            .rev()
            .take_while(|&byte| !byte.is_ascii_whitespace())
            .count()
    }

    /// The number of bytes between the cursor and the first whitespace
    /// character after it.
    fn _non_whitespace_bytes_post(&self) -> usize {
        self.line[self.bytes as usize..]
            .bytes()
            .take_while(|&byte| !byte.is_ascii_whitespace())
            .count()
    }

    /// The current word the cursor is embedded in, where a word is considered
    /// a collection of non-whitespace bytes.
    pub fn _word(&self) -> &'_ str {
        &self.line[self.bytes as usize - self.non_whitespace_bytes_pre()
            ..self.bytes as usize + self._non_whitespace_bytes_post()]
    }

    /// The part of the word the cursor is on that's before the cursor.
    pub fn word_pre(&self) -> &'_ str {
        &self.line[self.bytes as usize - self.non_whitespace_bytes_pre()
            ..self.bytes as usize]
    }

    /// The part of the word the cursor is on that's after the cursor.
    pub fn _word_post(&self) -> &'_ str {
        &self.line[self.bytes as usize
            ..self.bytes as usize + self._non_whitespace_bytes_post()]
    }
}

fn _get_matched_bytes(line: &str, bytes_before_cursor: usize) -> usize {
    line[..bytes_before_cursor]
        .bytes()
        .rev()
        .take_while(|&byte| !byte.is_ascii_whitespace())
        .count()
}

#[cfg(test)]
mod tests {
    use super::_get_matched_bytes;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // `|`
    fn empty_line() {
        assert_eq!("".len(), _get_matched_bytes("", 0))
    }

    #[test]
    // `|foo`
    fn cursor_at_beginning_of_line() {
        assert_eq!("".len(), _get_matched_bytes("foo", 0))
    }

    #[test]
    // ` ⇥|foo`
    fn only_whitespace_before_cursor() {
        assert_eq!("".len(), _get_matched_bytes(" \tfoo", 2))
    }

    #[test]
    // `foo |bar`
    fn cursor_before_word() {
        assert_eq!("".len(), _get_matched_bytes("foo bar", 4))
    }

    #[test]
    // `foo | bar`
    fn cursor_between_spaces() {
        assert_eq!("".len(), _get_matched_bytes("foo  bar", 4))
    }

    #[test]
    // `foo⇥|⇥bar`
    fn cursor_between_tabs() {
        assert_eq!("".len(), _get_matched_bytes("foo\t\tbar", 4))
    }

    #[test]
    // `foo|`
    fn cursor_end_of_word() {
        assert_eq!("foo".len(), _get_matched_bytes("foo", 3))
    }

    #[test]
    // `foo|bar`
    fn cursor_inside_word() {
        assert_eq!("foo".len(), _get_matched_bytes("foobar", 3))
    }

    #[test]
    // `fö|ö` (every `ö` is 2 bytes long)
    fn cursor_inside_word_multibyte_chars() {
        assert_eq!("fö".len(), _get_matched_bytes("föö", 3))
    }
}
