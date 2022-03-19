use mlua::prelude::{Lua, LuaResult};
use neovim::{api::LogLevel, Neovim};
use std::sync::{Arc, Mutex};

use crate::autocmds;
use crate::State;

// TODO: try to attach to the current buffer.
/// Executed by the `CompleetStart` user command.
pub fn compleet_start(lua: &Lua, state: &Arc<Mutex<State>>) -> LuaResult<()> {
    let api = Neovim::new(lua)?.api;

    let _state = state.clone();
    let _state = &mut _state.lock().unwrap();

    if _state.augroup_id.is_none() {
        _state.augroup_id = Some(autocmds::setup(lua, &api, state)?);

        api.notify("[nvim-compleet]: Started completing.", LogLevel::Info)?;
    } else {
        api.notify(
            "[nvim-compleet]: Completion was already on.",
            LogLevel::Error,
        )?;
    }

    Ok(())
}
