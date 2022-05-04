use bindings::api;
use mlua::{prelude::LuaResult, Lua};

use crate::state::State;

// TODO: refactor

/// Executed every time a byte or a group of bytes in an attached buffer is
/// modified.
pub fn on_bytes(
    lua: &Lua,
    state: &mut State,
    bufnr: u16,
    changedtick: u32,
    start_row: u16,
    start_col: u16,
    rows_deleted: u16,
    bytes_deleted: u16,
    rows_added: u16,
    bytes_added: u16,
) -> LuaResult<Option<bool>> {
    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    // If this buffer is queued to be detached we return `true`, as explained
    // in `:h api-lua-detach`. The help docs also mention a `nvim_buf_detach`
    // function but it seems to have been removed.
    if state.should_detach(bufnr) {
        return Ok(Some(true));
    }

    // We only care about insert mode events.
    if api::get_mode(lua)?.0 != "i" {
        return Ok(None);
    }

    // If we've added or deleted a line we return early. If we've stayed on the
    // same line but we've deleted characters we only continue if the
    // `completion.while_deleting` option is set.
    if rows_added != 0
        || rows_deleted != 0
        || (bytes_deleted != 0 && !state.settings.completion.while_deleting)
    {
        return Ok(None);
    }

    // Update the cursor.
    let cursor = &mut state.cursor;

    cursor.row = start_row;
    cursor.line = get_current_line(lua, cursor.row)?;
    cursor.bytes = (start_col
        + if bytes_deleted != 0 { 0 } else { bytes_added })
        as usize;

    state.matched_bytes = cursor.word_pre().len();

    state.completions.clear();
    // #[cfg(debug)]
    // {
    //     debug_cursor_position(
    //         lua,
    //         start_row,
    //         start_col,
    //         rows_deleted,
    //         bytes_deleted,
    //         rows_added,
    //         bytes_added,
    //         cursor,
    //     )?;
    // }

    state.changedtick_last_seen = changedtick;
    state.did_on_bytes = true;

    let channel = state.channel.as_mut().expect("channel already created");

    // TODO: don't clone
    let cursor = std::sync::Arc::new(cursor.clone());

    let buffer =
        state.attached_buffers.get(&bufnr).map(|buf| buf.clone()).unwrap();

    channel.stop_tasks();
    channel.fetch_completions(cursor, changedtick, buffer);

    Ok(None)
}

fn get_current_line(lua: &Lua, current_row: u16) -> LuaResult<String> {
    let current_line = api::buf_get_lines(
        lua,
        0,
        current_row as i32,
        current_row as i32 + 1,
        false,
    )?
    .into_iter()
    .next()
    .expect("there's always at least 1 line in this range");

    Ok(current_line)
}

// #[cfg(debug)]
// fn debug_cursor_position(
//     lua: &Lua,
//     start_row: u32,
//     start_col: u32,
//     rows_deleted: u32,
//     bytes_deleted: u32,
//     rows_added: u32,
//     bytes_added: u32,
//     cursor: &compleet::cursor::Cursor,
// ) -> LuaResult<()> {
//     use crate::bindings::nvim;

//     nvim::print(lua, "----------------")?;
//     nvim::print(lua, format!("Start row: {start_row}"))?;
//     nvim::print(lua, format!("Start col: {start_col}"))?;
//     nvim::print(lua, format!("Rows deleted: {rows_deleted}"))?;
//     nvim::print(lua, format!("Bytes deleted: {bytes_deleted}"))?;
//     nvim::print(lua, format!("Rows added: {rows_added}"))?;
//     nvim::print(lua, format!("Bytes added: {bytes_added}"))?;
//     nvim::print(lua, "")?;

//     let mut current_line = cursor.line.clone();
//     current_line.insert(cursor.bytes as usize, '|');
//     nvim::print(lua, format!("Current row: {}", cursor.row))?;
//     nvim::print(lua, format!("Current bytes: {}", cursor.bytes))?;
//     nvim::print(
//         lua,
//         format!("Current line (`|` is cursor): '{current_line}'"),
//     )?;

//     Ok(())
// }
