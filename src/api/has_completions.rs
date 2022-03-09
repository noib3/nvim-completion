use mlua::{Lua, Result};

use crate::completion;
use crate::state::CompletionState;
use crate::Nvim;

/// Executed on every call to `require("compleet").has_completions()`.
pub fn has_completions(
    lua: &Lua,
    completion_state: &mut CompletionState,
) -> Result<bool> {
    let nvim = Nvim::new(lua)?;

    completion_state.current_line = nvim.get_current_line()?;
    completion_state.bytes_before_cursor = nvim.win_get_cursor(0)?.1;

    completion_state.matched_prefix =
        String::from(completion::get_matched_prefix(
            &completion_state.current_line,
            completion_state.bytes_before_cursor,
        ));

    completion_state.completion_items =
        completion::complete(&completion_state.matched_prefix);

    Ok(!completion_state.completion_items.is_empty())
}
