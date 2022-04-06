use mlua::prelude::{Lua, LuaResult};

use crate::state::State;
use crate::ui;
use crate::utils;

/// Called when the RPC channel gets closed.
pub fn on_exit(lua: &Lua, state: &mut State, exit_code: u32) -> LuaResult<()> {
    match exit_code {
        // Exit code 143 means the server received a SIGTERM. That happens when
        // the user quits Neovim, which is not an error.
        143 => {},

        // Every other exit code should be considered an error.
        num => {
            // Delete the augroup and all its autocommands.
            state.augroup.unset(lua)?;

            // Register all the attached buffers to be detached on the next
            // call to `on_bytes`.
            state.buffers_to_be_detached.extend::<Vec<u32>>(
                state.attached_buffers.iter().map(|b| b.number).collect(),
            );

            // Cleanup the UI.
            ui::cleanup(lua, &mut state.ui)?;

            // Echo a warning message to the user.
            utils::echowar(
                lua,
                format!("The server just quit with exit code \"{num}\""),
            )?;
        },
    };

    Ok(())
}
