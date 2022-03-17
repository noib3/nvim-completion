use mlua::{Lua, Result};
use neovim::Neovim;

use crate::ui::UI;

// refactor: make it a method on UI?

/// Executed on every `InsertLeft` event.
pub fn cleanup_ui(lua: &Lua, ui: &mut UI) -> Result<()> {
    let api = &Neovim::new(lua)?.api;

    if ui.completion_menu.is_visible() {
        ui.completion_menu.hide(api)?;

        // The details pane can only be visible if the completion menu is
        // visible.
        if ui.details_pane.is_visible() {
            ui.details_pane.hide(api)?;
        }
    }

    if ui.completion_hint.is_visible() {
        ui.completion_hint.erase(api)?;
    }

    Ok(())
}
