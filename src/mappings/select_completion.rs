use mlua::{Lua, Result};
use neovim::Neovim;

use crate::state::State;

/// Executed on `<Plug>(compleet-next-completion)` and
/// `<Plug>(compleet-prev-completion)`.
pub fn select_completion(
    lua: &Lua,
    state: &mut State,
    step: i8, // either 1 or -1
) -> Result<()> {
    if !state.ui.completion_menu.is_visible() {
        return Ok(());
    }

    let hint = &mut state.ui.completion_hint;
    let menu = &mut state.ui.completion_menu;
    let details = &mut state.ui.details_pane;
    let completions = &state.completions;

    let last_index = completions.len() - 1;
    let new_selected_index = match step {
        // Selecting the next completion
        1 => match menu.selected_completion {
            Some(index) if index == last_index => None,
            Some(index) => Some(index + 1),
            None => Some(0),
        },

        // Selecting the previous completion
        -1 => match menu.selected_completion {
            Some(index) if index == 0 => None,
            Some(index) => Some(index - 1),
            None => Some(last_index),
        },

        _ => unreachable!(),
    };

    let api = Neovim::new(lua)?.api;

    // Update the completion menu.
    menu.select_completion(lua, &api, new_selected_index)?;

    match new_selected_index {
        // No selected completion -> clear the completion hint and close the
        // details window.
        None => {
            hint.erase(&api)?;
            details.hide(&api)?;
        },

        Some(index) => {
            let completion = &completions[index];

            // Update the completion hint.
            if state.settings.show_hints && state.line.cursor_is_at_eol() {
                hint.set(
                    lua,
                    &api,
                    index,
                    state.line.bytes_before_cursor,
                    &completion.text[state.line.matched_prefix.len()..],
                )?;
            }

            // Update the details window.
            match &completion.details {
                None => details.hide(&api)?,
                Some(lines) => details.show(
                    lua,
                    &api,
                    lines,
                    menu.winid
                        .expect("The menu is visible so it has a window id"),
                    menu.dimensions
                        .expect("The menu is visible so it has a position"),
                )?,
            }
        },
    }

    Ok(())
}
