use mlua::{Lua, Result};

use crate::state::State;
use crate::Nvim;

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
    let completions = &state.completion.completion_items;

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

    let nvim = &Nvim::new(lua)?;

    // Update the completion menu
    menu.select_completion(lua, nvim, new_selected_index)?;

    match new_selected_index {
        None => {
            hint.erase(nvim)?;
            details.hide(nvim)?;
        },

        Some(index) => {
            let completion = &completions[index];

            // Update the completion hint.
            if state.settings.show_hints && state.completion.cursor_is_at_eol()
            {
                hint.set(
                    lua,
                    nvim,
                    index,
                    state.completion.bytes_before_cursor,
                    &completion.text[state.completion.matched_prefix.len()..],
                )?;
            }

            // Update the details pane.
            match &completion.details {
                None => details.hide(nvim)?,
                Some(lines) => details.show(
                    lua,
                    nvim,
                    lines,
                    menu.position
                        .as_ref()
                        .expect("The menu is visible so it has a position"),
                )?,
            }
        },
    }

    Ok(())
}
