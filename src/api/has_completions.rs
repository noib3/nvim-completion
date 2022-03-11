use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::CompletionState;

/// Executed on every call to `require("compleet").has_completions()`.
pub fn has_completions(
    lua: &Lua,
    completion_state: &mut CompletionState,
) -> Result<bool> {
    let api = Neovim::new(lua)?.api;

    completion_state.current_line = api.get_current_line()?;
    completion_state.bytes_before_cursor = api.win_get_cursor(0)?.1;

    completion_state.matched_prefix =
        String::from(completion::get_matched_prefix(
            &completion_state.current_line,
            completion_state.bytes_before_cursor,
        ));

    completion_state.completion_items =
        completion::complete(&completion_state.matched_prefix);

    Ok(!completion_state.completion_items.is_empty())
}
