use mlua::{Lua, Result};

use crate::completion::CompletionState;
use crate::ui::UIState;
use crate::{insertion, Nvim};

pub fn accept_completion(
    lua: &Lua,
    completion_state: &mut CompletionState,
    ui_state: &mut UIState,
) -> Result<()> {
    if let Some(selected_index) = ui_state.completion_menu.selected_index {
        let nvim = Nvim::new(lua)?;

        let line_after_cursor = &completion_state.current_line
            [completion_state.bytes_before_cursor..];

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
            // The end column (which `nvim_buf_set_text` interprets to be
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
    }

    Ok(())
}
