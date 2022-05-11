use bindings::{api, nvim, opinionated::Buffer};
use mlua::prelude::{Lua, LuaResult};

use crate::client::Client;
use crate::messages;

/// Attaches `nvim-compleet` to all the buffers.
pub fn attach_all(lua: &Lua, client: &mut Client) -> LuaResult<()> {
    if client.is_completion_on() {
        let current = Buffer::get_current(lua)?;

        return match client.is_buffer_attached(&current) {
            true => messages::echoerr!(lua, "Completion is already on"),
            false => self::attach_current(lua, client),
        };
    }

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    client.cancel_detach_all();

    // Set the augroup.
    client.augroup.set(lua)?;

    // Schedule a `BufEnter` event on this buffer to attach it.
    nvim::schedule(
        lua,
        lua.create_function(move |lua, ()| {
            api::exec_autocmds(lua, ["BufEnter"], lua.create_table()?)
        })?,
    )?;

    messages::echoinfo!(lua, "Started completion in all buffers")?;

    Ok(())
}

/// Attaches `nvim-compleet` to the current buffer.
pub fn attach_current(lua: &Lua, state: &mut Client) -> LuaResult<()> {
    let current = Buffer::get_current(lua)?;

    if state.is_buffer_attached(&current) {
        messages::echoerr!(lua, "Completion is already on in this buffer")?;
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
    messages::echoinfo!(lua, "Started completion in buffer '{current}'",)?;

    Ok(())
}
