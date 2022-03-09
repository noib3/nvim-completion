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
    let nvim = Nvim::new(lua)?;

    if !state.ui.completion_menu.is_visible() {
        return Ok(());
    }

    let last_index = state.completion.completion_items.len() - 1;
    let new_selected_index = match step {
        // Selecting the next completion
        1 => match state.ui.completion_menu.selected_index {
            Some(index) if index == last_index => None,
            Some(index) => Some(index + 1),
            None => Some(0),
        },

        // Selecting the previous completion
        -1 => match state.ui.completion_menu.selected_index {
            Some(index) if index == 0 => None,
            Some(index) => Some(index - 1),
            None => Some(last_index),
        },

        _ => unreachable!(),
    };

    state
        .ui
        .completion_menu
        .select_completion(&nvim, new_selected_index)?;

    if (state.completion.bytes_before_cursor
        == state.completion.current_line.len())
        && state.settings.show_hints
    {
        match new_selected_index {
            None => state.ui.completion_hint.erase(&nvim)?,
            Some(index) => {
                state.ui.completion_hint.set(
                    lua,
                    &nvim,
                    index,
                    state.completion.bytes_before_cursor,
                    &state.completion.completion_items[index].text
                        [state.completion.matched_prefix.len()..],
                )?;
            },
        }
    }

    Ok(())
}
