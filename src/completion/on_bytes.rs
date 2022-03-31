use std::sync::Arc;

use mlua::{prelude::LuaResult, Lua};
use neovim::{Api, Neovim};

use crate::state::State;

/// Executed every time a byte or a group of bytes in an attached buffer is
/// modified.
pub fn on_bytes(
    lua: &Lua,
    state: &mut State,
    bufnr: u32,
    start_row: u32,
    start_col: u32,
    rows_deleted: u32,
    bytes_deleted: u32,
    rows_added: u32,
    bytes_added: u32,
) -> LuaResult<Option<bool>> {
    // If this buffer is queued to be detached we return `true`, as explained
    // in `:h api-lua-detach`. The help docs also mention a `nvim_buf_detach`
    // function but it seems to have been removed.
    // NOTE: this is needed until https://github.com/neovim/neovim/issues/17874
    // gets addressed.
    if state.buffers_to_be_detached.contains(&bufnr) {
        state.buffers_to_be_detached.retain(|&b| b != bufnr);
        return Ok(Some(true));
    }

    let api = Neovim::new(lua)?.api;

    // We only care about insert mode events.
    if api.get_mode()?.0 != "i" {
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
    cursor.line = get_current_line(&api, cursor.row)?;
    cursor.bytes =
        start_col + if bytes_deleted != 0 { 0 } else { bytes_added };

    #[cfg(debug)]
    {
        debug_cursor_position(
            lua,
            start_row,
            start_col,
            rows_deleted,
            bytes_deleted,
            rows_added,
            bytes_added,
            cursor,
        )?;
    }

    // TODO: if `setting.ui.menu.autoshow` is false we look at
    // `settings.hint.enable`. If that is also false we can just return early,
    // otherwise we just need to compute the first completion.

    let c = Arc::new(cursor.clone());

    let completions = &mut state.completions;
    let handles = &mut state.handles;
    let runtime = state.runtime.as_ref().expect("Runtime already created");
    let tx = state.tx.as_ref().expect("Runtime already created");
    let rx = state.rx.as_mut().expect("Runtime already created");

    // Abort any previous task. I guess this only works if the task associated
    // w/ the handle reaches a `.await`, which it currently doesn't.

    // NOTE: tokio tasks (aka green threads) can only be cancelled when they
    // reach a `.await` breakpoint.

    // Abort all previous tasks.
    handles.iter().for_each(|handle| handle.abort());

    // handles.clear();
    // completions.clear();
    // for source in state
    //     .sources
    //     .get(&bufnr)
    //     .expect("The buffer is attached so it has sources")
    //     .iter()
    // {
    //     // TODO: avoid cloning here, wrap cursor in an Arc.
    //     let cr = c.clone();

    //     let s = source.clone();
    //     let t = tx.clone();

    //     let handle = runtime.spawn(async move {
    //         let comps = s.complete(&cr).await;
    //         if let Err(_) = t.send(comps).await {
    //             println!("receiver dropped");
    //             return;
    //         }
    //     });

    //     state.handles.push(handle);
    // }

    // while let Some(comps) = &mut rx.recv() {
    //     completions.append(comps);
    // }

    state.did_on_bytes = true;

    Ok(None)
}

fn get_current_line(api: &Api, current_row: u32) -> LuaResult<String> {
    let current_line = api
        .buf_get_lines(
            0,
            current_row,
            (current_row + 1).try_into().unwrap(),
            false,
        )?
        .into_iter()
        .nth(0)
        .expect("There's always at least 1 line in this range");

    Ok(current_line)
}

#[cfg(debug)]
fn debug_cursor_position(
    lua: &Lua,
    start_row: u32,
    start_col: u32,
    rows_deleted: u32,
    bytes_deleted: u32,
    rows_added: u32,
    bytes_added: u32,
    cursor: &crate::completion::Cursor,
) -> LuaResult<()> {
    let nvim = Neovim::new(lua)?;

    nvim.print("----------------")?;
    nvim.print(format!("Start row: {start_row}"))?;
    nvim.print(format!("Start col: {start_col}"))?;
    nvim.print(format!("Rows deleted: {rows_deleted}"))?;
    nvim.print(format!("Bytes deleted: {bytes_deleted}"))?;
    nvim.print(format!("Rows added: {rows_added}"))?;
    nvim.print(format!("Bytes added: {bytes_added}"))?;
    nvim.print("")?;

    let mut current_line = cursor.line.clone();
    current_line.insert(cursor.bytes as usize, '|');
    nvim.print(format!("Current row: {}", cursor.row))?;
    nvim.print(format!("Current bytes: {}", cursor.bytes))?;
    nvim.print(format!("Current line (`|` is cursor): '{current_line}'"))?;

    Ok(())
}
