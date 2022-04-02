use mlua::{prelude::LuaResult, Lua};

use super::ui::Ui;

/// Executed on `InsertLeave` in attached buffers.
pub fn cleanup(lua: &Lua, ui: &mut Ui) -> LuaResult<()> {
    if ui.menu.floater.is_open() {
        ui.menu.floater.close(lua)?;

        // The details window is open if a completion is selected, which can
        // only happen if the menu is open.
        if ui.details.floater.is_open() {
            ui.details.floater.close(lua)?;
        }
    }

    if ui.hint.is_visible() {
        ui.hint.erase(lua)?;
    }

    Ok(())
}
