use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed by the `require("compleet").has_completions` Lua function.
pub fn has_completions(lua: &Lua, state: &mut State) -> Result<bool> {
    // If the augroup id is `None` that means the user has turned off the
    // plugin with `:CompleetStop`.
    if state.augroup_id.is_none() {
        return Ok(false);
    }

    let api = Neovim::new(lua)?.api;

    let cursor = &mut state.cursor;
    let completions = &mut state.completions;

    cursor.update_bytes(&api)?;
    cursor.update_line(&api)?;

    *completions = completion::complete(&cursor.line, cursor.bytes as usize);

    Ok(!completions.is_empty())
}
