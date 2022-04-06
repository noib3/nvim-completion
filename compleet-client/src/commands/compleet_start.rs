use mlua::prelude::{Lua, LuaResult};

use crate::{bindings::api, ui::Buffer, utils, State};

/// Attaches `nvim-compleet` to all the buffers.
pub fn attach_all(lua: &Lua, state: &mut State) -> LuaResult<()> {
    if state.augroup.is_set() {
        let current = Buffer::get_current(lua)?;
        // If the current buffer is not attached we try to attach it.
        if !state.attached_buffers.contains(&current) {
            self::attach_current(lua, state)?;
        }
        // If it is we echo an error message.
        else {
            utils::echoerr(lua, "Completion is already on")?;
        }
        return Ok(());
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    state.buffers_to_be_detached.clear();

    // Set the augroup.
    state.augroup.set(lua)?;

    // Trigger a `BufEnter` event on this buffer to attach it.
    api::exec_autocmds(
        lua,
        ["BufEnter"],
        lua.create_table_from([("buffer", 0)])?,
    )?;

    utils::echoinfo(lua, "Started completion in all buffers")?;

    Ok(())
}

/// Attaches `nvim-compleet` to the current buffer.
pub fn attach_current(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let current = Buffer::get_current(lua)?;

    if state.attached_buffers.contains(&current) {
        utils::echoerr(lua, "Completion is already on in this buffer")?;
        return Ok(());
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    // If this buffer was queued to be detached from buffer update events (the
    // ones setup by `nvim_buf_attach`, not autocommands) now it no longer
    // needs to.
    state
        .buffers_to_be_detached
        .retain(|&b| b != current.number);

    // Set the the augroup if it wasn't already set.
    if !state.augroup.is_set() {
        state.augroup.set(lua)?;
    }

    // Trigger a `BufEnter` event on this buffer to attach it.
    api::exec_autocmds(
        lua,
        ["BufEnter"],
        lua.create_table_from([("buffer", 0)])?,
    )?;

    // TODO: only display this if we've successfully attached to the buffer.
    utils::echoinfo(lua, format!("Started completion in buffer {current}"))?;

    Ok(())
}
