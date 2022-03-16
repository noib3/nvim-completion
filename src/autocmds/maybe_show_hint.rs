use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed on every `CursorMovedI` event.
pub fn maybe_show_hint(lua: &Lua, state: &mut State) -> Result<()> {
    if !state.settings.show_hints {
        return Ok(());
    }

    let api = Neovim::new(lua)?.api;

    state.line.update_bytes_before_cursor(&api)?;
    state.line.update_text(&api)?;

    // If the cursor is at the end of the line, check if there are completions
    // to be shown. If there are, show the hint for the first one.
    if state.line.cursor_is_at_eol() {
        state.line.update_matched_prefix()?;
        state.completions = completion::complete(&state.line.matched_prefix);

        if let Some(completion) = state.completions.first() {
            state.ui.completion_hint.set(
                lua,
                &api,
                0,
                state.line.bytes_before_cursor,
                &completion.text[state.line.matched_prefix.len()..],
            )?;
        }
    }

    Ok(())
}
