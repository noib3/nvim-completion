use mlua::{Lua, Result};

use crate::state::State;
use crate::Nvim;

/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> Result<()> {
    if !state.ui.completion_menu.is_visible()
        && !state.completion.completion_items.is_empty()
    {
        let nvim = &Nvim::new(lua)?;
        state.ui.completion_menu.show_completions(
            lua,
            nvim,
            &state.completion.completion_items,
        )?;
    }

    Ok(())
}
