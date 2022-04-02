use compleet::completion::Completions;
use mlua::prelude::{Lua, LuaResult};

use crate::state::State;

/// Executed when new completions arrive to the channel.
pub fn update(
    lua: &Lua,
    state: &mut State,
    new: Option<Completions>,
) -> LuaResult<()> {
    let hint = &mut state.ui.as_mut().unwrap().hint;
    let menu = &mut state.ui.as_mut().unwrap().menu;
    let details = &mut state.ui.as_mut().unwrap().details;

    // Update the selected completion.
    Ok(())
}
