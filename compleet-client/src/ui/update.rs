use std::cmp;

use compleet::completion::Completions;
use mlua::prelude::{Lua, LuaResult};

use super::floater::RelativeTo;
use crate::state::State;

// TODO: what we send in `on_bytes` should be a request, not a notification.
//
// TODO: handle `state.completions` being non empty.
//
// TODO: how do we cleanup the UI if after sometime the server tells us no
// completions were found?.
//
//
/// Executed when new completions arrive to the channel.
pub fn update(
    lua: &Lua,
    state: &mut State,
    new: Completions,
) -> LuaResult<()> {
    let ui = &mut state.ui;
    let settings = &state.settings;

    // TODO
    if let Some(old) = ui.menu.selected_index {
        ui.menu.selected_index = Some(cmp::min(old, new.len() - 1));
    }

    // Update the completion hint.
    if settings.ui.hint.enable && state.cursor.is_at_eol() {
        let i = ui.menu.selected_index.unwrap_or(0);
        ui.hint.set(lua, &new[i], &state.cursor)?;
    } else if ui.hint.is_visible {
        ui.hint.erase(lua)?;
    }

    // Try to position the completion menu.
    let (position, height, width) = match super::menu::find_position(
        lua,
        &new,
        &ui.menu.floater,
        settings.ui.menu.max_height,
    )? {
        Some((row, col, h, w)) => (RelativeTo::Cursor(row, col), h, w),

        // If it wasn't possible to position the menu we close it and also
        // close the details window, then return.
        None => {
            ui.menu.floater.close(lua)?;
            ui.details.floater.close(lua)?;
            return Ok(());
        },
    };

    // Open the menu.
    ui.menu.floater.open(lua, position, height, width)?;

    // Fill the menu's buffer.
    let lines = new
        .iter()
        .map(|completion| completion.format.clone())
        .collect::<Vec<String>>();

    ui.menu.fill(lua, lines)?;

    state.completions = new;

    Ok(())
}
