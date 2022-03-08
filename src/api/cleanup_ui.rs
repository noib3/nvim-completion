use mlua::{Lua, Result};

use crate::state::UIState;
use crate::Nvim;

/// Executed on both `CursorMovedI` and `InsertLeft`.
pub fn cleanup_ui(lua: &Lua, ui: &mut UIState) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    if ui.completion_menu.is_visible() {
        ui.completion_menu.hide(&nvim)?;

        // The details pane can only be visible if the completion menu is
        // visible.
        if ui.details_pane.is_visible() {
            ui.details_pane.hide(&nvim)?;
        }
    }

    if ui.completion_hint.is_visible() {
        ui.completion_hint.erase(&nvim)?;
    }

    Ok(())
}
