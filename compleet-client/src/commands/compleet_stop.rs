use bindings::opinionated::Buffer;
use mlua::prelude::{Lua, LuaResult};

use crate::ui;
use crate::utils;
use crate::State;

/// Detaches `nvim-compleet` from all the buffers.
pub fn detach_all(lua: &Lua, state: &mut State) -> LuaResult<()> {
    if !state.augroup.is_set() {
        utils::echoerr(lua, "Completion is already off")?;
        return Ok(());
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    state.detach_all_buffers();

    // Delete the augroup containing all the autocommands.
    state.augroup.unset(lua)?;

    // Cleanup the UI in case the user has somehow executed `CompleetStop!`
    // without exiting insert mode (for example via an autocommand. Unlikely
    // but possible).
    ui::cleanup(lua, &mut state.ui)?;

    state.completions.clear();

    utils::echoinfo(lua, "Stopped completion in all buffers")?;

    Ok(())
}

/// Detaches `nvim-compleet` from the current buffer.
pub fn detach_current(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let current = Buffer::get_current(lua)?;

    if !state.is_buffer_attached(&current) {
        utils::echoerr(lua, "Completion is already off in this buffer")?;
        return Ok(());
    }

    state.detach_buffer(&current);

    // Delete all the buffer-local autocommands on this buffer.
    state.augroup.clear_local(lua, current.bufnr)?;

    ui::cleanup(lua, &mut state.ui)?;

    state.completions.clear();

    utils::echoinfo(lua, format!("Stopped completion in buffer {current}"))?;

    Ok(())
}
