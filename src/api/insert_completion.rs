use mlua::{Lua, Result};
use neovim::Neovim;

use crate::state::State;

/// Executed on both `<Plug>(compleet-insert-hinted-completion)` and
/// `<Plug>(compleet-insert-selected-completion)`.
pub fn insert_completion(
    lua: &Lua,
    state: &mut State,
    selected_index: usize,
) -> Result<()> {
    let api = Neovim::new(lua)?.api;

    let line = &state.line;
    let completions = &mut state.completions;

    // TODO: this doesn't work for right-to-left languages.
    let line_after_cursor = &line.text[line.bytes_before_cursor..];

    let selected_completion = &completions[selected_index];

    let (start_col, replacement) = get_completion(
        &line.matched_prefix,
        line_after_cursor,
        &selected_completion.text,
    );

    let start_col =
        line.bytes_before_cursor - line.matched_prefix.len() + start_col;

    let shift_the_cursor_this_many_bytes =
        selected_completion.text.len() - line.matched_prefix.len();

    let current_row = api.win_get_cursor(0)?.0;

    api.buf_set_text(
        0,
        current_row - 1,
        start_col,
        current_row - 1,
        // The end column (which `Api::buf_set_text` interprets to be
        // bytes from the beginning of the line, not characters) is
        // always equal to `bytes_before_cursor`, meaning we never
        // mangle the text after the current cursor position.
        line.bytes_before_cursor,
        &[replacement],
    )?;

    let new_column =
        line.bytes_before_cursor + shift_the_cursor_this_many_bytes;

    api.win_set_cursor(0, current_row, new_column)?;

    completions.clear();

    // We don't do any UI cleanup here (e.g. `completion_menu.hide()`, etc.)
    // since inserting a completion will move the cursor, triggering a
    // `CursorMovedI` event, which in turn executes `api::cleanup_ui` where
    // the cleanup happens.

    Ok(())
}

// Time: O(min(completion.len() - matched_prefix.len(), line_after_cursor.len()))
// Space: O(1)
fn get_completion<'a>(
    matched_prefix: &'a str,
    line_after_cursor: &'a str,
    completion: &'a str,
) -> (usize, &'a str) {
    // We don't care about the first `matched_prefix.len()` bytes of the
    // completion text since we're not doing any error correction yet.
    //
    // NOTE: this should never panic since `completion` is expected to always
    // be strictly longer than `matched_prefix`.
    let completion_wo_prefix = &completion[matched_prefix.len()..];

    let bytes_after_cursor = line_after_cursor.len();
    let bytes_rest_of_completion = completion_wo_prefix.len();
    let mut take_this_many_bytes_from_completion = bytes_rest_of_completion;

    // TODO: can probably be condensed/written more elegantly without the outer
    // `if` and with a single for loop in the range
    // `0..cmp::min(bytes_after_cursor, bytes_rest_of_completion)`,
    // but this'll do // for now.
    //
    // E.g. `f|ar`, completion is `foobar`.
    if bytes_after_cursor < bytes_rest_of_completion {
        for j in (0..bytes_after_cursor).rev() {
            if completion_wo_prefix.ends_with(&line_after_cursor[..j + 1]) {
                take_this_many_bytes_from_completion -= j + 1;
                break;
            }
        }
    // E.g. `foo|arbaz`, completion is `foobar`.
    } else {
        for i in 0..bytes_rest_of_completion {
            if line_after_cursor.starts_with(&completion_wo_prefix[i..]) {
                take_this_many_bytes_from_completion = i;
                break;
            }
        }
    }

    (
        matched_prefix.len(),
        &completion_wo_prefix[..take_this_many_bytes_from_completion],
    )
}

#[cfg(test)]
mod tests {
    use super::get_completion;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // Line: `foo|`
    // Completion: `foobar`
    // ->
    // Inserted: `bar`
    // Result: `foobar`
    fn foo1() {
        assert_eq!((3, "bar"), get_completion("foo", "", "foobar"));
    }

    #[test]
    // Line: `foo|baz`
    // Completion: `foobar`
    // ->
    // Inserted: `bar`
    // Result: `foobarbaz`
    fn foo2() {
        assert_eq!((3, "bar"), get_completion("foo", "baz", "foobar"));
    }

    #[test]
    // Line: `foo|ar`
    // Completion: `foobar`
    // ->
    // Inserted: `b`
    // Result: `foobar`
    fn foo3() {
        assert_eq!((3, "b"), get_completion("foo", "ar", "foobar"));
    }

    #[test]
    // Line: `föö|ár`
    // Completion: `fööbár`
    // ->
    // Inserted: `b`
    // Result: `fööbár`
    fn foo4() {
        assert_eq!((5, "b"), get_completion("föö", "ár", "fööbár"));
    }

    #[test]
    // Line: `foo|arbaz`
    // Completion: `foobar`
    // ->
    // Inserted: `b`
    // Result: `foobarbaz`
    fn foo5() {
        assert_eq!((3, "b"), get_completion("foo", "arbaz", "foobar"));
    }

    #[test]
    // Line: `foo|bar`
    // Completion: `foobar`
    // ->
    // Inserted: ``
    // Result: `foobar`
    fn foo6() {
        assert_eq!((3, ""), get_completion("foo", "bar", "foobar"));
    }

    #[test]
    // Line: `foo|r`
    // Completion: `foobar`
    // ->
    // Inserted: `ba`
    // Result: `foobar`
    fn foo7() {
        assert_eq!((3, "ba"), get_completion("foo", "r", "foobar"));
    }

    #[test]
    // Line: `f|ar`
    // Completion: `foobar`
    // ->
    // Inserted: `oob`
    // Result: `foobar`
    fn foo8() {
        assert_eq!((1, "oob"), get_completion("f", "ar", "foobar"));
    }

    #[test]
    // Line: `f|r`
    // Completion: `foobar`
    // ->
    // Inserted: `ooba`
    // Result: `foobar`
    fn foo9() {
        assert_eq!((1, "ooba"), get_completion("f", "r", "foobar"));
    }

    #[test]
    // Line: `f|rbaz`
    // Completion: `foobar`
    // ->
    // Inserted: `ooba`
    // Result: `foobarbaz`
    fn foo10() {
        assert_eq!((1, "ooba"), get_completion("f", "rbaz", "foobar"));
    }
}
