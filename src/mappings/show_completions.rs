use mlua::{Lua, Result};
use neovim::Neovim;

use crate::state::State;
use crate::ui::completion_menu;

// TODO: refactor.
/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> Result<()> {
    let completions = &state.completions;
    let menu = &mut state.ui.completion_menu;

    if !menu.is_visible() && !completions.is_empty() {
        // TODO: already select first completion.
        let api = Neovim::new(lua)?.api;

        let position = menu.show_completions(
            &api,
            completions,
            state.settings.max_menu_height,
        )?;

        if let Some(pos) = &position {
            menu.winid = Some(completion_menu::create_floatwin(
                lua, &api, menu.bufnr, pos,
            )?);

            // TODO: that -2 is ugly.
            completion_menu::fill_buffer(
                lua,
                &api,
                menu.bufnr,
                pos.width - 2,
                menu.matched_chars_nsid,
                completions,
            )?;
        }
    }

    Ok(())
}
