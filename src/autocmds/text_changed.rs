use mlua::{Lua, Result};
use neovim::Neovim;

use crate::completion;
use crate::state::State;

/// Executed on every `TextChangedI` event.
pub fn text_changed(lua: &Lua, state: &mut State) -> Result<()> {
    let api = Neovim::new(lua)?.api;

    let line = &mut state.line;
    let completions = &mut state.completions;
    let ui = &mut state.ui;

    line.update_bytes_before_cursor(&api)?;
    line.update_text(&api)?;
    line.update_matched_prefix()?;

    *completions = completion::complete(&line.matched_prefix);

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
    if state.settings.show_hints && line.cursor_is_at_eol() {
        ui.completion_hint.set(
            lua,
            &api,
            0,
            line.bytes_before_cursor,
            &completions[0].text[line.matched_prefix.len()..],
        )?;
    }

    Ok(())
}
