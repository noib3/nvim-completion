use mlua::{Lua, Result};
use neovim::{LogLevel, Neovim};
use std::sync::{Arc, Mutex};

use crate::autocmds;
use crate::State;

/// Executed by the `CompleetStart` user command.
pub fn compleet_start(lua: &Lua, state: &Arc<Mutex<State>>) -> Result<()> {
    let nvim = Neovim::new(lua)?;
    let empty = lua.create_table()?;

    let _state = state.clone();
    let _state = &mut _state.lock().unwrap();

    if _state.augroup_id.is_none() {
        _state.augroup_id = Some(autocmds::setup(lua, &nvim.api, state)?);

        nvim.notify(
            "[nvim-compleet]: Started completing.",
            LogLevel::Info,
            empty,
        )?;
    } else {
        nvim.notify(
            "[nvim-compleet]: Completion was already on.",
            LogLevel::Error,
            empty,
        )?;
    }

    Ok(())
}
