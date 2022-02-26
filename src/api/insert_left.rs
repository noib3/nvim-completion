use mlua::{Lua, Result};

use crate::ui::UIState;
use crate::Nvim;

/// Executed on every `InsertLeft` event.
pub fn insert_left(lua: &Lua, ui_state: &mut UIState) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    ui_state.completion_menu.hide(&nvim)?;
    ui_state.completion_hint.erase(&nvim)?;
    ui_state.details_pane.hide(&nvim)?;
    // TODO: reset selected index?

    Ok(())
}
