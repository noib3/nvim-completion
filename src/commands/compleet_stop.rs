use mlua::prelude::LuaResult;
use neovim::api::{Api, LogLevel};

use crate::State;

/// Executed by the `CompleetStop` user command.
pub fn compleet_stop(api: &Api, state: &mut State) -> LuaResult<()> {
    if let Some(id) = state.augroup_id {
        // Delete the augroup containing all our autocmds.
        api.del_augroup_by_id(id)?;

        // Cleanup the UI in case the user has somehow executed `CompleetStop`
        // without exiting insert mode (for example via an autocmd. Unlikely
        // but possible).
        state.ui.cleanup(api)?;

        state.augroup_id = None;

        // Notify the user and fuck off.
        api.notify("[nvim-compleet]: Stopped completing.", LogLevel::Info)?;
    } else {
        api.notify(
            "[nvim-compleet]: Completion was already off.",
            LogLevel::Error,
        )?;
    }

    Ok(())
}
