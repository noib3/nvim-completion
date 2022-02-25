use mlua::{Lua, Result, Table};

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

        let current_buffer = nvim.get_current_buf()?;
        let current_window = nvim.get_current_win()?;
        let current_row = nvim.win_get_cursor(current_window)?.0 - 1;

        let (start_col, replacement) = insertion::get_completion(
            &completion_state.matched_prefix,
            line_after_cursor,
            &completion_state.completion_items[selected_index].text,
        );

        let start_col = completion_state.bytes_before_cursor
            - completion_state.matched_prefix.len()
            + start_col;

        // The end column (which `nvim_buf_set_text` interprets to be
        // bytes from the beginning of the line, not characters) is
        // always equal to `bytes_before_cursor`, meaning we never
        // mangle the text after the current cursor position.
        let end_col = completion_state.bytes_before_cursor;

        let shift_the_cursor_this_many_bytes =
            completion_state.completion_items[selected_index].text.len()
                - completion_state.matched_prefix.len();

        nvim.buf_set_text(
            current_buffer,
            current_row,
            start_col,
            current_row,
            end_col,
            &[replacement.to_string()],
        )?;

        // TODO: fix this
        let pos: Table = lua.create_table()?;
        pos.set::<usize, usize>(1, current_row + 1)?;
        pos.set::<usize, usize>(
            2,
            completion_state.bytes_before_cursor
                + shift_the_cursor_this_many_bytes,
        )?;
        nvim.win_set_cursor(current_window, pos)?;

        completion_state.completion_items.clear();
    }

    Ok(())
}
