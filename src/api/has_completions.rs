use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed by the `require("compleet").has_completions` Lua function.
pub fn has_completions(lua: &Lua, state: &mut State) -> Result<bool> {
    let api = Neovim::new(lua)?.api;

    let buffer = &mut state.buffer;
    let completions = &mut state.completions;

    buffer.get_bytes_before_cursor(&api)?;
    buffer.get_text(&api)?;

    *completions = completion::complete(&buffer.line, buffer.at_bytes);

    Ok(!completions.is_empty())
}
