use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;

/// Executed by the `require("compleet").has_completions` Lua function.
pub fn has_completions(lua: &Lua, state: &mut State) -> LuaResult<bool> {
    let api = Neovim::new(lua)?.api;

    // If the buffer is not attached we return early.
    if !state.attached_buffers.contains(&api.get_current_buf()?) {
        return Ok(false);
    }

    let cursor = &mut state.cursor;
    let completions = &mut state.completions;

    cursor.at_bytes = api.win_get_cursor(0)?.1;
    cursor.line = api.get_current_line()?;

    completions.clear();
    for source in state.sources.iter() {
        completions.append(&mut source.complete(&api, &cursor)?);
    }

    Ok(!completions.is_empty())
}
