use mlua::{Lua, Result};

use crate::completion;
use crate::state::State;
use crate::Nvim;

/// Executed on both `CursorMovedI` and `InsertEnter`.
pub fn maybe_show_hint(lua: &Lua, state: &mut State) -> Result<()> {
    if !state.settings.show_hints {
        return Ok(());
    }

    let nvim = Nvim::new(lua)?;

    state.completion.update_bytes_before_cursor(&nvim)?;
    state.completion.update_current_line(&nvim)?;

    // If hints are enabled and the cursor is at the end of the line, check if
    // there are completions to be shown.
    if state.completion.cursor_is_at_eol() {
        state.completion.update_matched_prefix()?;
        state.completion.completion_items =
            completion::complete(&state.completion.matched_prefix);

        if let Some(item) = state.completion.completion_items.first() {
            state.ui.completion_hint.set(
                lua,
                &nvim,
                0,
                state.completion.bytes_before_cursor,
                &item.text[state.completion.matched_prefix.len()..],
            )?;
        }
    }

    Ok(())
}
