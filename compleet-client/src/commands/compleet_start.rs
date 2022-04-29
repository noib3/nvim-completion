use bindings::{api, nvim, opinionated::Buffer};
use mlua::prelude::{Lua, LuaResult};

use crate::{utils, State};

/// Attaches `nvim-compleet` to all the buffers.
pub fn attach_all(lua: &Lua, state: &mut State) -> LuaResult<()> {
    if state.augroup.is_set() {
        let current = Buffer::get_current(lua)?;
        // If the current buffer is not attached we try to attach it.
        if !state.is_buffer_attached(&current) {
            self::attach_current(lua, state)?;
        }
        // If it is we echo an error message.
        else {
            utils::echoerr(lua, "Completion is already on")?;
        }
        return Ok(());
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    state.cancel_detach_all();

    // Set the augroup.
    state.augroup.set(lua)?;

    // Schedule a `BufEnter` event on this buffer to attach it.
    nvim::schedule(
        lua,
        lua.create_function(move |lua, ()| {
            api::exec_autocmds(lua, ["BufEnter"], lua.create_table()?)
        })?,
    )?;

    utils::echoinfo(lua, "Started completion in all buffers")?;

    Ok(())
}

/// Attaches `nvim-compleet` to the current buffer.
pub fn attach_current(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let current = Buffer::get_current(lua)?;

    if state.is_buffer_attached(&current) {
        utils::echoerr(lua, "Completion is already on in this buffer")?;
        return Ok(());
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    state.cancel_detach_buffer(&current);

    // Set the the augroup if it wasn't already set.
    if !state.augroup.is_set() {
        state.augroup.set(lua)?;
    }

    // Schedule a `BufEnter` event on this buffer to attach it.
    nvim::schedule(
        lua,
        lua.create_function(move |lua, ()| {
            api::exec_autocmds(lua, ["BufEnter"], lua.create_table()?)
        })?,
    )?;

    // TODO: only display this if we've successfully attached to the buffer.
    utils::echoinfo(lua, format!("Started completion in buffer '{current}'"))?;

    Ok(())
}
