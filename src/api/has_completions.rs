use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed by the `require("compleet").has_completions` Lua function.
pub fn has_completions(lua: &Lua, state: &mut State) -> LuaResult<bool> {
    // TODO: check if the bufnr is in the attached buffers instead.
    // If the augroup id is `None` that means the user has turned off the
    // plugin with `:CompleetStop`.
    if state.augroup_id.is_none() {
        return Ok(false);
    }

    let api = Neovim::new(lua)?.api;

    let cursor = &mut state.cursor;
    let completions = &mut state.completions;

    cursor.update_at_bytes(&api)?;
    cursor.update_line(&api)?;
    cursor.update_matched_bytes();

    *completions = completion::complete(&cursor);

    Ok(!completions.is_empty())
}
