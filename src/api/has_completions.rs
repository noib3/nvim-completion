use std::sync::Arc;

use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;

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

    let c = Arc::new(cursor.clone());

    let completions = &mut state.completions;
    let handles = &mut state.handles;
    let runtime = state.runtime.as_ref().expect("Runtime already created");
    let tx = state.tx.as_ref().expect("Runtime already created");
    let rx = state.rx.as_mut().expect("Runtime already created");

    // Abort all previous tasks.
    handles.iter().for_each(|handle| handle.abort());

    handles.clear();
    completions.clear();
    for source in state
        .sources
        .get(&bufnr)
        .expect("The buffer is attached so it has sources")
        .iter()
    {
        // TODO: avoid cloning cursor, wrap it in an Arc.
        let cr = c.clone();

        let s = source.clone();
        let t = tx.clone();

        let handle = runtime.spawn(async move {
            let comps = s.complete(&cr).await;
            if let Err(_) = t.send(comps).await {
                println!("receiver dropped");
                return;
            }
        });

        state.handles.push(handle);
    }

    // TODO: make this work
    // while let Some(comps) = &mut rx.recv().await {
    //     completions.append(comps);
    // }

    Ok(!completions.is_empty())
}
