use mlua::prelude::{Lua, LuaResult};

use crate::state::State;

/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> LuaResult<()> {
    // let menu = &mut state.ui.completion_menu;
    // let completions = &state.completions;

    // if !menu.is_visible() && !completions.is_empty() {
    //     let maybe_position = menu::positioning::get_position(
    //         lua,
    //         completions,
    //         &state.settings.ui.menu,
    //     )?;

    //     if let Some(position) = maybe_position {
    //         menu.spawn(lua, &position, &state.settings.ui.menu.border)?;
    //         menu.fill(lua, completions)?;
    //     }
    // }

    Ok(())
}
