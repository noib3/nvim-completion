use std::cmp;

use bindings::api;
use mlua::prelude::{Lua, LuaResult};
use sources::prelude::Completions;

use super::floater::RelativeTo;
use crate::{state::State, ui};

// TODO: refactor everything

// BUGS:
// 1. selecting a completion while the UI is waiting for the last source before
//    being cleanup up panics.

/// Scheduled when a source sends its completions to the channel.
pub fn update(
    lua: &Lua,
    state: &mut State,
    new: Completions,
    changedtick: u32,
    has_last: bool,
) -> LuaResult<()> {
    // A source sending no completions should usually cause no UI update. The
    // only exception is if that was the last source for a given `changedtick`,
    // and that `changedtick` is newer that the last one that caused a UI
    // update. This means there are no completions available and the UI should
    // be cleaned up.
    if new.is_empty() {
        if has_last && changedtick != state.changedtick_last_update {
            ui::cleanup(lua, &mut state.ui)?;
        }
        return Ok(());
    }

    let ui = &mut state.ui;
    let settings = &state.settings;
    let completions = &mut state.completions;

    // Update the contents of the completion menu.
    if completions.is_empty() {
        ui.menu.fill(lua, &new)?;
    } else {
        ui.menu.insert(lua, &new, 0)?;
    }

    // Extend the already present completions with the new ones.
    // NOTE: for now they are added to the end of the vector. Once we have more
    // sources we'll have to do some sorting.
    completions.extend(new);

    // Update the selected completion (if there was one).
    if let Some(old) = ui.menu.selected_index {
        ui.menu.selected_index = Some(cmp::min(old, completions.len() - 1));
    }

    // Update the completion hint.
    if settings.ui.hint.enable && state.cursor.is_at_eol() {
        let i = ui.menu.selected_index.unwrap_or(0);
        ui.hint.set(lua, &completions[i], &state.cursor)?;
    }

    // TODO: respect `settings.menu.autoshow`.

    // Try to position the completion menu.
    let (position, height, width) = match super::menu::find_position(
        lua,
        &completions,
        &ui.menu.floater,
        settings.ui.menu.max_height,
    )? {
        Some((row, col, h, w)) => (RelativeTo::Cursor(row, col), h, w),

        // If it wasn't possible to position the menu we close it and also
        // close the details window, then return.
        None => {
            ui.menu.floater.close(lua)?;
            ui.details.floater.close(lua)?;
            ui.menu.selected_index = None;
            return Ok(());
        },
    };

    // Open the menu.
    if let Some(winid) = ui.menu.floater.id {
        ui.menu.floater.r#move(lua, position, height, width)?;

        // Reset the cursor to the first row of the window.
        // TODO: document why.
        api::win_set_cursor(lua, winid, 1, 0)?;

        if let Some(index) = ui.menu.selected_index {
            // Moving the floater resets the value of the `cursorline` option.
            // If a completion is selected it needs to be turned
            // back on.
            api::win_set_option(lua, winid, "cursorline", true)?;

            // Reset the cursor to the row of the selected completion.
            api::win_set_cursor(lua, winid, index as u16 + 1, 0)?;

            // Update the completion details.
            ui.details.update(
                lua,
                completions.get(index),
                &ui.menu.floater,
                true,
            )?;
        } else {
            ui.details.floater.close(lua)?;
        }
    } else {
        ui.menu.floater.open(lua, position, height, width)?;
    }

    state.changedtick_last_update = changedtick;

    Ok(())
}
