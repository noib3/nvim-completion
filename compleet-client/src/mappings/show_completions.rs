use mlua::prelude::{Lua, LuaResult};

use crate::state::State;
use crate::ui::{floater::RelativeTo, menu};

/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let menu = &mut state.ui.menu;

    if !menu.floater.is_open() && !state.completions.is_empty() {
        let (position, height, width) = match menu::find_position(
            lua,
            &state.completions,
            &menu.floater,
            state.settings.ui.menu.max_height,
        )? {
            Some((row, col, h, w)) => (RelativeTo::Cursor(row, col), h, w),
            None => {
                return Ok(());
            },
        };

        menu.floater.open(lua, position, height, width)?;
        menu.fill(lua, &state.completions)?;
    }

    Ok(())
}
