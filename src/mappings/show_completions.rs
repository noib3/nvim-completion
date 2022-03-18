use mlua::{Lua, Result};
use neovim::Neovim;

use crate::state::State;
use crate::ui::menu;

/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> Result<()> {
    let menu = &mut state.ui.completion_menu;
    let completions = &state.completions;

    if !menu.is_visible() && !completions.is_empty() {
        // TODO: already select the first completion.
        let api = Neovim::new(lua)?.api;

        let maybe_position = menu::positioning::get_position(
            &api,
            completions,
            state.settings.max_menu_height,
        )?;

        if let Some(position) = maybe_position {
            menu.spawn(lua, &api, &position)?;
            menu.fill(lua, &api, completions)?;
        }
    }

    Ok(())
}
