use mlua::{Lua, Result};
use neovim::{Api, Neovim};

use crate::state::State;

/// Executed every time some bytes in the current buffer are changed.
pub fn bytes_changed(
    lua: &Lua,
    state: &mut State,
    start_row: usize,
    start_col: usize,
    rows_deleted: usize,
    bytes_deleted: usize,
    rows_added: usize,
    bytes_added: usize,
) -> Result<()> {
    let api = Neovim::new(lua)?.api;

    // TODO: detach buffer on every `InsertLeave` and re-attach it on every
    // `InsertEnter`?
    //
    // We only care about insert mode events.
    if api.get_mode()?.0 != "i" {
        return Ok(());
    }

    // If we've added a new line, deleted one or stayed in the same line but
    // deleted characters we don't do anything. We only do something when new
    // characters are added to the current line.
    if rows_added != 0 || rows_deleted != 0 || bytes_deleted != 0 {
        return Ok(());
    }

    let buffer = &mut state.buffer;

    buffer.row = start_row;
    buffer.line = get_current_line(&api, buffer.row)?;
    buffer.at_bytes = start_col + bytes_added;

    // // Used for debugging.
    // let nvim = Neovim::new(lua)?;
    // nvim.print(format!("Start row: {start_row}"))?;
    // nvim.print(format!("Start col: {start_col}"))?;
    // nvim.print(format!("Rows deleted: {rows_deleted}"))?;
    // nvim.print(format!("Bytes deleted: {bytes_deleted}"))?;
    // nvim.print(format!("Rows added: {rows_added}"))?;
    // nvim.print(format!("Bytes added: {bytes_added}"))?;

    // // Used for debugging.
    // let nvim = Neovim::new(lua)?;
    // let mut current_line = buffer.line.clone();
    // current_line.insert(buffer.bytes_before_cursor, '|');
    // nvim.print(format!("Current row: {}", buffer.row))?;
    // nvim.print(format!("Current line (`|` is cursor): '{current_line}'"))?;

    let completions = &mut state.completions;
    *completions = super::complete(&buffer.line, buffer.at_bytes);

    if completions.is_empty() {
        return Ok(());
    }

    let settings = &state.settings;
    let ui = &mut state.ui;

    // Update the completion menu.
    if settings.autoshow_menu {
        ui.draw_instructions.menu_position = ui
            .completion_menu
            .show_completions(&api, &completions, settings.max_menu_height)?;
    }

    // If hints are enabled and the cursor is at the end of the line, set the
    // hint for the first completion.
    if settings.show_hints && buffer.cursor_is_at_eol() {
        ui.draw_instructions.hinted_index = Some(0);
    }

    Ok(())
}

fn get_current_line(api: &Api, current_row: usize) -> Result<String> {
    let current_line = api
        .buf_get_lines(
            0,
            current_row,
            isize::try_from(current_row + 1).unwrap(),
            false,
        )?
        .into_iter()
        .nth(0)
        .expect("There's always at least 1 line in this range");

    Ok(current_line)
}

// fn get_current_row(
//     start_row: usize,
//     rows_deleted: usize,
//     rows_added: usize,
// ) -> usize {
//     start_row + if rows_deleted != 0 { 0 } else { rows_added }
// }

// fn get_bytes_before_cursor(
//     current_line: &str,
//     start_col: usize,
//     rows_deleted: usize,
//     bytes_deleted: usize,
//     rows_added: usize,
//     bytes_added: usize,
// ) -> usize {
//     if rows_deleted != 0 {
//         current_line.len()
//     } else if rows_added != 0 {
//         0
//     } else {
//         start_col + if bytes_deleted != 0 { 0 } else { bytes_added }
//     }
// }
