use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed on every `TextChangedI` event.
pub fn text_changed(lua: &Lua, state: &mut State) -> Result<()> {
    let api = Neovim::new(lua)?.api;

    let buffer = &mut state.buffer;
    let completions = &mut state.completions;
    let ui = &mut state.ui;

    buffer.get_bytes_before_cursor(&api)?;
    buffer.get_text(&api)?;

    *completions =
        completion::complete(&buffer.line, buffer.bytes_before_cursor);

    if completions.is_empty() {
        return Ok(());
    }

    // Update the completion menu.
    if state.settings.autoshow_menu {
        ui.completion_menu.show_completions(
            lua,
            &api,
            &completions,
            state.settings.max_menu_height,
        )?;
    }

    // If hints are enabled and the cursor is at the end of the line, show the
    // hint for the first completion.
    if state.settings.show_hints && buffer.cursor_is_at_eol() {
        ui.completion_hint.set(
            lua,
            &api,
            0,
            buffer.bytes_before_cursor,
            &completions[0].text[buffer.matched_prefix.len()..],
        )?;
    }

    Ok(())
}
