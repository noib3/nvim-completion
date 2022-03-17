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

    let buffer = &mut state.buffer;

    buffer.get_bytes_before_cursor(&api)?;
    buffer.get_text(&api)?;

    // If the cursor is at the end of the line, check if there are completions
    // to be shown. If there are, show the hint for the first one.
    if buffer.cursor_is_at_eol() {
        state.completions =
            completion::complete(&buffer.line, buffer.bytes_before_cursor);

        if let Some(completion) = state.completions.first() {
            state.ui.completion_hint.set(
                lua,
                &api,
                0,
                buffer.row,
                buffer.bytes_before_cursor,
                &completion.text[completion.todo..],
            )?;
        }
    }

    Ok(())
}
