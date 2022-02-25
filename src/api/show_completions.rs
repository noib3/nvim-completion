use mlua::{Lua, Result};

use crate::completion::CompletionState;
use crate::ui::UIState;
use crate::Nvim;

// TODO:
// #[compleet::lua_export]
pub fn show_completions(
    lua: &Lua,
    completion_state: &CompletionState,
    ui_state: &mut UIState,
) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    if ui_state.completion_menu.is_visible()
        || completion_state.completion_items.is_empty()
    {
        return Ok(());
    }

    ui_state.completion_menu.show_completions(
        &nvim,
        lua,
        &completion_state.completion_items,
    )?;

    Ok(())
}
