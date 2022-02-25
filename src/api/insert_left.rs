use mlua::{Lua, Result};

use crate::ui::UIState;
use crate::Nvim;

pub fn insert_left(lua: &Lua, ui_state: &mut UIState) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    ui_state.completion_menu.hide(&nvim)?;
    ui_state.details_pane.hide(&nvim)?;
    ui_state.virtual_text.erase(&nvim)?;
    // TODO: reset selected index?

    Ok(())
}
