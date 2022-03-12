use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed on every call to `require("compleet").has_completions()`.
pub fn has_completions(lua: &Lua, state: &mut State) -> Result<bool> {
    let api = Neovim::new(lua)?.api;

    state.line.update_bytes_before_cursor(&api)?;
    state.line.update_text(&api)?;
    state.line.update_matched_prefix()?;

    state.completions = completion::complete(&state.line.matched_prefix);

    Ok(!state.completions.is_empty())
}
