use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;
use crate::ui::details;

/// Executed on `<Plug>(compleet-next-completion)` and
/// `<Plug>(compleet-prev-completion)`.
pub fn select_completion(
    lua: &Lua,
    state: &mut State,
    step: i8, // either 1 or -1
) -> LuaResult<()> {
    if !state.ui.completion_menu.is_visible() {
        return Ok(());
    }

    let api = Neovim::new(lua)?.api;

    let menu = &mut state.ui.completion_menu;
    let hint = &mut state.ui.completion_hint;
    let details = &mut state.ui.completion_details;

    let cursor = &state.cursor;
    let completions = &state.completions;

    let last_index = completions.len() - 1;
    let new_selected_index = match step {
        // Selecting the next completion
        1 => match menu.selected_index {
            Some(index) if index == last_index => None,
            Some(index) => Some(index + 1),
            None => Some(0),
        },

        // Selecting the previous completion
        -1 => match menu.selected_index {
            Some(index) if index == 0 => None,
            Some(index) => Some(index - 1),
            None => Some(last_index),
        },

        _ => unreachable!(),
    };

    // Update the completion menu.
    menu.select(&api, new_selected_index)?;

    if let Some(index) = new_selected_index {
        let completion = &completions[index];

        // Update the completion hint.
        if state.settings.show_hints && state.cursor.is_at_eol() {
            let text = &completion.text[completion.matched_prefix_len..];
            hint.set(lua, &api, text, cursor.row, cursor.bytes, index)?;
        }

        // Update the completion details.
        if let Some(lines) = &completion.details {
            let menu_winid = menu
                .winid
                .expect("The menu is visible so it has a window id");

            let menu_width =
                menu.width.expect("The menu is visible so it has a width");

            let maybe_position = details::positioning::get_position(
                &api, lines, menu_winid, menu_width,
            )?;

            if let Some(position) = &maybe_position {
                if details.is_visible() {
                    details.shift(lua, &api, menu_winid, position)?;
                } else {
                    details.spawn(lua, &api, menu_winid, position)?;
                }
                details.fill(&api, lines)?;
            } else {
                details.close(&api)?;
            }
        } else {
            details.close(&api)?;
        }
    } else {
        // No new selected completion -> erase the completion hint and close
        // the details window.
        hint.erase(&api)?;
        details.close(&api)?;
    }

    Ok(())
}
