use mlua::prelude::{Lua, LuaFunction, LuaResult};

use crate::autocmds::Augroup;
use crate::bindings::nvim;
use crate::ui::Buffer;
use crate::utils;
use crate::State;

/// Executed by the `CompleetStart{!}` user commands.
pub fn compleet_start(
    lua: &Lua,
    state: &mut State,
    bang: bool,
) -> LuaResult<()> {
    // `CompleetStart!` attaches to all the buffers, while `CompleetStart` only
    // attaches to the current buffer.
    match bang {
        true => self::attach_all(lua, state),
        false => self::attach_current(lua, state),
    }
}

/// Attaches `nvim-compleet` to all the buffers.
fn attach_all(lua: &Lua, state: &mut State) -> LuaResult<()> {
    // If the autocmd for the `BufEnter` event is already set then completion
    // is in general already on. If it's disabled in the current buffer we try
    // to attach, if not we echo an error message.
    if state.augroup.is_buf_enter_set() {
        let current = Buffer::get_current(lua)?;
        match state.attached_buffers.contains(&current) {
            true => utils::echoerr(lua, "Completion is already on")?,
            false => self::attach_current(lua, state)?,
        }
        return Ok(());
    }

    state.buffers_to_be_detached.clear();

    // Recreate the `Compleet` augroup.
    state.augroup = Augroup::new(lua)?;

    // Add the `BufEnter` autocmd.
    let on_buf_enter = lua.registry_value::<LuaFunction>(
        state
            .on_buf_enter_key
            .as_ref()
            .expect("`on_buf_enter` has already been created"),
    )?;

    state.augroup.add_autocmds(
        lua,
        None,
        vec![("BufEnter", on_buf_enter.clone())],
    )?;

    // NOTE: We can't call `on_buf_enter` here or the state's RefCell would
    // panic. Instead we schedule it for a later time in Neovim's event loop
    // via `vim.schedule`.
    nvim::schedule(lua, on_buf_enter)?;

    utils::echoinfo(lua, "Started completion in all buffers")?;

    Ok(())
}

/// Attaches `nvim-compleet` to the current buffer.
fn attach_current(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let current = Buffer::get_current(lua)?;

    if state.attached_buffers.contains(&current) {
        utils::echoerr(lua, "Completion is already on in this buffer")?;
        return Ok(());
    }

    // If this buffer was queued to be detached from buffer update events (the
    // ones setup by `nvim_buf_attach`, not autocmds) now it no longer needs
    // to.
    state
        .buffers_to_be_detached
        .retain(|&b| b != current.number);

    // If the augroup is off we need to recreate it.
    if !state.augroup.is_active() {
        state.augroup = Augroup::new(lua)?;
    }

    // Schedule a `on_buf_enter` to attach to the current buffer.
    let on_buf_enter = lua.registry_value::<LuaFunction>(
        state
            .on_buf_enter_key
            .as_ref()
            .expect("`try_buf_attach` has already been created"),
    )?;
    nvim::schedule(lua, on_buf_enter)?;

    // TODO: only display this once we've successfully attached to the
    // buffer.
    utils::echoinfo(
        lua,
        format!("Started completion in buffer {}", current.number),
    )?;

    Ok(())
}
