use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;

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

    let menu = &mut state.ui.completion_menu;
    let completions = &state.completions;

    let last_index = completions.len() - 1;
    let new_index = match step {
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

    let api = Neovim::new(lua)?.api;

    let hint = &mut state.ui.completion_hint;
    let details = &mut state.ui.completion_details;
    let cursor = &state.cursor;

    // Select the new completion.
    menu.select(&api, new_index)?;

    // Update the completion hint.
    if state.settings.ui.hint.enable && cursor.is_at_eol() {
        let new_completion_w_i = new_index.map(|i| (&completions[i], i));
        hint.update(lua, &api, new_completion_w_i, cursor)?;
    }

    // Update the completion details.
    let menu_winid = menu
        .winid
        .expect("The menu is visible so it has a window id");

    let menu_width =
        menu.width.expect("The menu is visible so it has a width");

    let lines = new_index.and_then(|i| completions[i].details.as_ref());

    details.update(lua, &api, lines, menu_width, menu_winid)?;

    Ok(())
}
