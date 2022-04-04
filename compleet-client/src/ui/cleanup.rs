use mlua::{prelude::LuaResult, Lua};

use super::ui::Ui;

/// Closes the completion menu and the details window if they are open, erases
/// the completion hint if it's visible.
pub fn cleanup(lua: &Lua, ui: &mut Ui) -> LuaResult<()> {
    if ui.menu.floater.is_open() {
        ui.menu.floater.close(lua)?;

        // The details window can only be open if a completion is selected,
        // which in turn can only happen if the menu is open.
        if ui.details.floater.is_open() {
            ui.details.floater.close(lua)?;
        }
    }

    if ui.hint.is_visible {
        ui.hint.erase(lua)?;
    }

    Ok(())
}
