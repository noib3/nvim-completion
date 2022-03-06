use mlua::{Lua, Result};

use crate::config::Config;
use crate::state::{CompletionState, UIState};
use crate::Nvim;

pub fn select_completion(
    lua: &Lua,
    config: &Config,
    ui_state: &mut UIState,
    completion_state: &CompletionState,
    step: i8, // either 1 or -1
) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    if !ui_state.completion_menu.is_visible() {
        return Ok(());
    }

    let last_index = completion_state.completion_items.len() - 1;
    let new_selected_index = match step {
        // Selecting the next completion
        1 => match ui_state.completion_menu.selected_index {
            Some(index) if index == last_index => None,
            Some(index) => Some(index + 1),
            None => Some(0),
        },

        // Selecting the previous completion
        -1 => match ui_state.completion_menu.selected_index {
            Some(index) if index == 0 => None,
            Some(index) => Some(index - 1),
            None => Some(last_index),
        },

        _ => unreachable!(),
    };

    ui_state
        .completion_menu
        .select_completion(&nvim, new_selected_index)?;

    if (completion_state.bytes_before_cursor
        == completion_state.current_line.len())
        && config.show_hints
    {
        match new_selected_index {
            None => ui_state.completion_hint.erase(&nvim)?,
            Some(index) => {
                // TODO: this shouldn't be needed at this point
                let current_row = nvim.win_get_cursor(0)?.0;
                ui_state.completion_hint.set(
                    lua,
                    &nvim,
                    index,
                    current_row - 1,
                    completion_state.bytes_before_cursor,
                    &completion_state.completion_items[index].text
                        [completion_state.matched_prefix.len()..],
                )?;
            },
        }
    }

    Ok(())
}
