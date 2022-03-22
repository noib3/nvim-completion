use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;

/// Executed on both `<Plug>(compleet-insert-hinted-completion)` and
/// `<Plug>(compleet-insert-selected-completion)`.
pub fn insert_completion(
    lua: &Lua,
    state: &mut State,
    index: usize,
) -> LuaResult<()> {
    let completion = &state.completions[index];
    let cursor = &state.cursor;

    let text_to_insert = get_text_to_insert(
        cursor.matched_bytes as usize,
        &cursor.line[cursor.at_bytes as usize..],
        &completion.text,
    );

    let end_column = (cursor.at_bytes - cursor.matched_bytes) as usize
        + completion.text.len();

    // NOTE: Inserting the completion in the buffer right at this point
    // triggers `completion::bytes_changed`, which causes the Mutex wrapping
    // the global state to deadlock (I don't really understand why right now
    // tbh).
    //
    // To avoid this we wrap the call to `api.buf_set_text` in a closure and
    // pass it to `nvim.schedule`. This seems to solve the issue.
    //
    // TODO: Understand why this happens.

    let insert_completion = lua.create_function(
        move |lua, (row, col, text): (u32, u32, String)| {
            let api = Neovim::new(lua)?.api;
            api.buf_set_text(0, row, col, row, col, &[text])?;
            api.win_set_cursor(0, row + 1, end_column as u32)?;
            Ok(())
        },
    )?;

    let nvim = Neovim::new(lua)?;

    nvim.schedule(insert_completion.bind((
        cursor.row,
        cursor.at_bytes,
        text_to_insert.to_string(),
    ))?)?;

    // Reset the selected completion.
    state.ui.completion_menu.selected_index = None;

    Ok(())
}

/// Returns the text that should be inserted into the buffer, taking into
/// account what comes after the cursor. For example, if we have `f|o` and
/// we're completing `foo` we only need to insert the first `o`, since the
/// other one is already present in the buffer.
fn get_text_to_insert<'a>(
    matched_bytes: usize,
    line_after_cursor: &'a str,
    completion: &'a str,
) -> &'a str {
    // We don't care about the first `matched_bytes` bytes of the completion
    // text since we're not doing any error correction yet.
    //
    // NOTE: this should never panic since `completion` is expected to always
    // be strictly longer than `matched_bytes`.
    let completion_wo_prefix = &completion[matched_bytes..];

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

    &completion_wo_prefix[..take_this_many_bytes_from_completion]
}

#[cfg(test)]
mod tests {
    use super::get_text_to_insert;

    // NOTE: the `|` in the following comments indicates the cursor position.

    #[test]
    // Line: `foo|`
    // Completion: `foobar`
    // ->
    // Inserted: `bar`
    // Result: `foobar`
    fn foo1() {
        assert_eq!("bar", get_text_to_insert("foo".len(), "", "foobar"));
    }

    #[test]
    // Line: `foo|baz`
    // Completion: `foobar`
    // ->
    // Inserted: `bar`
    // Result: `foobarbaz`
    fn foo2() {
        assert_eq!("bar", get_text_to_insert("foo".len(), "baz", "foobar"));
    }

    #[test]
    // Line: `foo|ar`
    // Completion: `foobar`
    // ->
    // Inserted: `b`
    // Result: `foobar`
    fn foo3() {
        assert_eq!("b", get_text_to_insert("foo".len(), "ar", "foobar"));
    }

    #[test]
    // Line: `föö|ár`
    // Completion: `fööbár`
    // ->
    // Inserted: `b`
    // Result: `fööbár`
    fn foo4() {
        assert_eq!("b", get_text_to_insert("föö".len(), "ár", "fööbár"));
    }

    #[test]
    // Line: `foo|arbaz`
    // Completion: `foobar`
    // ->
    // Inserted: `b`
    // Result: `foobarbaz`
    fn foo5() {
        assert_eq!("b", get_text_to_insert("foo".len(), "arbaz", "foobar"));
    }

    #[test]
    // Line: `foo|bar`
    // Completion: `foobar`
    // ->
    // Inserted: ``
    // Result: `foobar`
    fn foo6() {
        assert_eq!("", get_text_to_insert("foo".len(), "bar", "foobar"));
    }

    #[test]
    // Line: `foo|r`
    // Completion: `foobar`
    // ->
    // Inserted: `ba`
    // Result: `foobar`
    fn foo7() {
        assert_eq!("ba", get_text_to_insert("foo".len(), "r", "foobar"));
    }

    #[test]
    // Line: `f|ar`
    // Completion: `foobar`
    // ->
    // Inserted: `oob`
    // Result: `foobar`
    fn foo8() {
        assert_eq!("oob", get_text_to_insert("f".len(), "ar", "foobar"));
    }

    #[test]
    // Line: `f|r`
    // Completion: `foobar`
    // ->
    // Inserted: `ooba`
    // Result: `foobar`
    fn foo9() {
        assert_eq!("ooba", get_text_to_insert("f".len(), "r", "foobar"));
    }

    #[test]
    // Line: `f|rbaz`
    // Completion: `foobar`
    // ->
    // Inserted: `ooba`
    // Result: `foobarbaz`
    fn foo10() {
        assert_eq!("ooba", get_text_to_insert("f".len(), "rbaz", "foobar"));
    }
}
