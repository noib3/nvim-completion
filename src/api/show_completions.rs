use mlua::{Lua, Result};
use neovim::Neovim;

use crate::state::State;

/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> Result<()> {
    if !state.ui.completion_menu.is_visible() && !state.completions.is_empty()
    {
        let api = Neovim::new(lua)?.api;
        state.ui.completion_menu.show_completions(
            lua,
            &api,
            &state.completions,
        )?;
    }

    Ok(())
}
