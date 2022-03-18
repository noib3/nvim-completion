use mlua::{Lua, Result};
use neovim::{LogLevel, Neovim};

use crate::State;

// TODO: detach from every buffer we've attached to.
/// Executed by the `CompleetStop` user command.
pub fn compleet_stop(lua: &Lua, state: &mut State) -> Result<()> {
    let nvim = Neovim::new(lua)?;
    let api = &nvim.api;

    let empty = lua.create_table()?;

    if let Some(id) = state.augroup_id {
        // Delete the augroup containing all our autocmds.
        api.del_augroup_by_id(id)?;

        // Cleanup the UI in case the user has somehow executed `CompleetStop`
        // without exiting insert mode (for example via an autocmd. Unlikely
        // but possible).
        state.ui.cleanup(api)?;

        state.augroup_id = None;

        // Notify the user and fuck off.
        nvim.notify(
            "[nvim-compleet]: Stopped completing.",
            LogLevel::Info,
            empty,
        )?;
    } else {
        nvim.notify(
            "[nvim-compleet]: Completion was already off.",
            LogLevel::Error,
            empty,
        )?;
    }

    Ok(())
}
