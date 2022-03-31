use std::sync::Arc;

use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;

// TODO: even tough this has to block the thread in runs in, it shouldn't block
// the UI thread. An `InsertLeave` event, inserting or deleting characters
// should make this return `false` immediately.

/// Executed by the `require("compleet").has_completions` Lua function.
pub fn has_completions(lua: &Lua, state: &mut State) -> LuaResult<bool> {
    let api = Neovim::new(lua)?.api;

    // If the buffer is not attached it can't have any completions.
    let bufnr = api.get_current_buf()?;
    if !state.attached_buffers.contains(&bufnr) {
        return Ok(false);
    }

    let cursor = &mut state.cursor;

    cursor.bytes = api.win_get_cursor(0)?.1;
    cursor.line = api.get_current_line()?;

    // TODO: make cursor an arc
    let c = Arc::new(cursor.clone());

    let completions = &mut state.completions;
    let runtime = state.runtime.as_ref().expect("Runtime already created");

    // Abort all previous tasks.
    state.handles.iter().for_each(|handle| handle.abort());
    state.handles.clear();

    // Get handles to the new ones.
    let handles = state
        .sources
        .get(&bufnr)
        .expect("The buffer is attached so it has sources")
        .iter()
        .map(|source| {
            let s = source.clone();
            let curs = c.clone();
            runtime.spawn(async move { s.complete(&curs).await })
        })
        .collect::<Vec<_>>();

    completions.clear();
    runtime.block_on(async {
        for handle in handles {
            if let Ok(comps) = handle.await {
                completions.extend(comps);
            }
        }
    });

    Ok(!completions.is_empty())
}
