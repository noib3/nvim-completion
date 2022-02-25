use mlua::{Lua, Result};

use crate::ui::UIState;
use crate::Nvim;

pub fn select_next_completion(
    lua: &Lua,
    ui_state: &mut UIState,
    completion_items_len: usize,
) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    if !ui_state.completion_menu.is_visible() {
        return Ok(());
    }

    let new_selected_index = match ui_state.completion_menu.selected_index {
        Some(index) if index == completion_items_len - 1 => None,
        Some(index) => Some(index + 1),
        None => Some(0),
    };

    ui_state
        .completion_menu
        .select_completion(&nvim, new_selected_index)?;

    Ok(())
}
