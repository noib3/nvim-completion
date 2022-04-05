use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;
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
            // Remove all the autocmds.
            if let Some(id) = state.augroup_id {
                api::del_augroup_by_id(lua, id)?;
                state.augroup_id = None;
            }

            // Detach all the attached buffers.
            state
                .buffers_to_be_detached
                .append(&mut state.attached_buffers);

            // Cleanup the UI.
            ui::cleanup(lua, &mut state.ui)?;

            // Echo an error message to the user.
            utils::echoerr(
                lua,
                vec![
                    ("The server just quit with exit code ", None),
                    (&num.to_string(), Some("Visual")),
                ],
            )?;
        },
    };

    Ok(())
}
