use mlua::{Lua, Result};
use neovim::{LogLevel, Neovim};

use crate::autocmds;
use crate::State;

// TODO: detach from every buffer we've attached to.
/// Executed by the `CompleetStop` user command.
pub fn compleet_stop(lua: &Lua, state: &mut State) -> Result<()> {
    let nvim = Neovim::new(lua)?;
    let empty = lua.create_table()?;

    if let Some(id) = state.augroup_id {
        // Delete the augroup containing all our autocmds.
        nvim.api.del_augroup_by_id(id)?;

        // Cleanup the UI in case the user has somehow executed `CompleetStop`
        // without exiting insert mode (for example via an autocmd. Unlikely
        // but possible).
        autocmds::cleanup_ui(lua, &mut state.ui)?;

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
