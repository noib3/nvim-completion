use mlua::prelude::{Lua, LuaResult};

use crate::ui;
use crate::ui::Buffer;
use crate::utils;
use crate::State;

/// Detaches `nvim-compleet` from all the buffers.
pub fn detach_all(lua: &Lua, state: &mut State) -> LuaResult<()> {
    if !state.augroup.is_set() {
        utils::echoerr(lua, "Completion is already off")?;
        return Ok(());
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    // Move all the buffer numbers from the `attached_buffers` vector to
    // `buffers_to_be_detached`.
    state.buffers_to_be_detached.extend::<Vec<u16>>(
        state.attached_buffers.iter().map(|b| b.number).collect(),
    );

    // Clear the vector of attached buffers.
    state.attached_buffers.clear();

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

    if !state.attached_buffers.contains(&current) {
        utils::echoerr(lua, "Completion is already off in this buffer")?;
        return Ok(());
    }

    // Remove the buffer from `attached_buffers` and schedule it to be
    // detached.
    state.attached_buffers.retain(|b| b != &current);
    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    state.buffers_to_be_detached.push(current.number);

    // Delete all the buffer-local autocommands on this buffer.
    state.augroup.clear_local(lua, &current)?;

    ui::cleanup(lua, &mut state.ui)?;

    state.completions.clear();

    utils::echoinfo(lua, format!("Stopped completion in buffer {current}"))?;

    Ok(())
}
