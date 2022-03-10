use mlua::{Lua, Result};
// use std::cmp;

use crate::completion;
use crate::state::State;
use crate::Nvim;

/// Executed on every `TextChangedI` event.
pub fn text_changed(lua: &Lua, state: &mut State) -> Result<()> {
    let nvim = &Nvim::new(lua)?;

    state.completion.update_bytes_before_cursor(nvim)?;
    state.completion.update_current_line(nvim)?;

    state.completion.matched_prefix =
        String::from(completion::get_matched_prefix(
            &state.completion.current_line,
            state.completion.bytes_before_cursor,
        ));

    state.completion.completion_items =
        completion::complete(&state.completion.matched_prefix);

    if state.completion.completion_items.is_empty() {
        state.ui.completion_menu.selected_completion = None;
        return Ok(());
    }

    // TODO: I don't actually need this bc the completion_menu is hidden on
    // every cursor_moved and every completion_menu.hide() already sets the
    // selected index to None. Maybe I do if I decide it's not the
    // responsability of completion_menu.hide() to reset the selected index.
    // if let Some(index) = state.ui.completion_menu.selected_index {
    //     state.ui.completion_menu.selected_index =
    //         Some(cmp::min(index, state.completion.completion_items.len() - 1))
    // }

    if state.settings.autoshow_menu {
        state.ui.completion_menu.show_completions(
            lua,
            nvim,
            &state.completion.completion_items,
        )?;
    }

    // Only show a completion hint if there's no text in the line beyond the
    // current cursor position (and if hints are enabled, ofc).
    if (state.completion.bytes_before_cursor
        == state.completion.current_line.len())
        && state.settings.show_hints
    {
        state.ui.completion_hint.set(
            lua,
            nvim,
            0,
            state.completion.bytes_before_cursor,
            &state.completion.completion_items[0].text
                [state.completion.matched_prefix.len()..],
        )?;
    }

    Ok(())
}
