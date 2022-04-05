use mlua::prelude::{Lua, LuaResult};

use crate::state::State;

/// Executed on `<Plug>(compleet-{prev,next}-completion)`.
pub fn select_completion(
    lua: &Lua,
    state: &mut State,
    step: i8, // either 1 or -1
) -> LuaResult<()> {
    let ui = &mut state.ui;

    // If the completion menu isn't open this is a no-op.
    if !ui.menu.floater.is_open() {
        return Ok(());
    }

    let last_index = state.completions.len() - 1;
    let new_index = match step {
        // +1 => selecting the next completion.
        1 => match ui.menu.selected_index {
            Some(index) if index == last_index => None,
            Some(index) => Some(index + 1),
            None => Some(0),
        },

        // -1 => selecting the previous completion.
        -1 => match ui.menu.selected_index {
            Some(index) if index == 0 => None,
            Some(index) => Some(index - 1),
            None => Some(last_index),
        },

        _ => unreachable!(),
    };

    // Select the new completion.
    ui.menu.select(lua, new_index)?;

    // Update the details window.
    let maybe = new_index.and_then(|i| state.completions.get(i));
    ui.details.update(lua, maybe, &ui.menu.floater, false)?;

    // Update the completion hint.
    if state.settings.ui.hint.enable && state.cursor.is_at_eol() {
        match maybe {
            Some(completion) => ui.hint.set(lua, completion, &state.cursor)?,
            None => ui.hint.erase(lua)?,
        }
    }

    Ok(())
}
