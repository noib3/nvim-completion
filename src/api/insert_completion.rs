use mlua::{Lua, Result};

use crate::state::CompletionState;
use crate::{insertion, Nvim};

pub fn insert_completion(
    lua: &Lua,
    completion_state: &mut CompletionState,
    selected_index: usize,
) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    let line_after_cursor =
        &completion_state.current_line[completion_state.bytes_before_cursor..];

    let (start_col, replacement) = insertion::get_completion(
        &completion_state.matched_prefix,
        line_after_cursor,
        &completion_state.completion_items[selected_index].text,
    );

    let start_col = completion_state.bytes_before_cursor
        - completion_state.matched_prefix.len()
        + start_col;

    let shift_the_cursor_this_many_bytes =
        completion_state.completion_items[selected_index].text.len()
            - completion_state.matched_prefix.len();

    let current_row = nvim.win_get_cursor(0)?.0;

    nvim.buf_set_text(
        0,
        current_row - 1,
        start_col,
        current_row - 1,
        // The end column (which `Nvim::buf_set_text` interprets to be
        // bytes from the beginning of the line, not characters) is
        // always equal to `bytes_before_cursor`, meaning we never
        // mangle the text after the current cursor position.
        completion_state.bytes_before_cursor,
        &[replacement],
    )?;

    let new_column = completion_state.bytes_before_cursor
        + shift_the_cursor_this_many_bytes;

    nvim.win_set_cursor(0, &[current_row, new_column])?;

    completion_state.completion_items.clear();

    // We don't do any UI cleanup here (e.g. `completion_menu.hide()`, etc.)
    // since inserting a completion will move the cursor, triggering a
    // `CursorMovedI` event, which in turns executes `api::cursor_moved` where
    // the cleanup happens.

    Ok(())
}
