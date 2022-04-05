use mlua::prelude::{Lua, LuaResult};

use crate::ui;
use crate::ui::Buffer;
use crate::utils;
use crate::State;

/// Executed by the `CompleetStop` user command.
pub fn compleet_stop(
    lua: &Lua,
    state: &mut State,
    bang: bool,
) -> LuaResult<()> {
    // The `CompleetStop!` command detaches all the buffers, while
    // `CompleetStop` only detaches the current buffer. To detach a buffer we
    // need to return `true` the next time the `on_bytes` function is called.
    match bang {
        true => self::detach_all(lua, state),
        false => self::detach_current(lua, state),
    }
}

/// Detaches `nvim-compleet` from all the buffers.
fn detach_all(lua: &Lua, state: &mut State) -> LuaResult<()> {
    if !state.augroup.is_active() {
        utils::echoerr(lua, "Completion is already off")?;
        return Ok(());
    }

    // Delete the  augroup containing all the autocmds.
    state.augroup.delete_all(lua)?;

    // Move all the buffer numbers from the `attached_buffers` vector to
    // `buffers_to_be_detached`.
    state.buffers_to_be_detached.extend::<Vec<u32>>(
        state.attached_buffers.iter().map(|b| b.number).collect(),
    );

    // Cleanup the UI in case the user has somehow executed
    // `CompleetStop!` without exiting insert mode (for example via an
    // autocmd. Unlikely but possible).
    ui::cleanup(lua, &mut state.ui)?;

    utils::echoinfo(lua, "Stopped completion in all buffers")?;

    Ok(())
}

/// Detaches `nvim-compleet` from the current buffer.
fn detach_current(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let current = Buffer::get_current(lua)?;

    if !state.attached_buffers.contains(&current) {
        utils::echoerr(lua, "Completion is already off in this buffer")?;
        return Ok(());
    }

    // Remove the current buffer
    state.attached_buffers.retain(|b| b != &current);
    state.buffers_to_be_detached.push(current.number);

    ui::cleanup(lua, &mut state.ui)?;

    // Delete all the buffer-local autocmds we had set for this buffer.
    state.augroup.delete_local(lua, &current)?;

    utils::echoinfo(
        lua,
        format!("Stopped completion for buffer {}", current.number),
    )?;

    Ok(())
}
