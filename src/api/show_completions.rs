use mlua::{Lua, Result};

use crate::state::State;
use crate::Nvim;

pub fn show_completions(lua: &Lua, state: &mut State) -> Result<()> {
    if !state.ui.completion_menu.is_visible()
        && !state.completion.completion_items.is_empty()
    {
        let nvim = Nvim::new(lua)?;
        state.ui.completion_menu.show_completions(
            &nvim,
            lua,
            &state.completion.completion_items,
        )?;
    }

    Ok(())
}
