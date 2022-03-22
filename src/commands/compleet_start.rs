use mlua::prelude::{Lua, LuaResult};
use neovim::{api::LogLevel, Neovim};
use std::sync::{Arc, Mutex};

use crate::State;

// TODO: try to attach to the current buffer.
/// Executed by the `CompleetStart` user command.
pub fn compleet_start(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    bang: bool,
) -> LuaResult<()> {
    let _state = state.clone();
    let _state = &mut _state.lock().unwrap();

    // The `CompleetStart!` command attaches to all the buffers, while
    // `CompleetStart` only attaches to the current buffer.
    match bang {
        true => attach_all_buffers(lua, _state),
        false => attach_current_buffer(lua, _state),
    }
}

/// TODO: docs
fn attach_all_buffers(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let nvim = Neovim::new(lua)?;
    let api = &nvim.api;

    if state.bufenter_autocmd_id.is_some() {
        api.notify(
            "[nvim-compleet] Completion is already on",
            LogLevel::Error,
        )?;
        return Ok(());
    }

    state.buffers_to_be_detached.clear();

    let trigger_try_attach = lua.create_function(move |lua, ()| {
        let api = Neovim::new(lua)?.api;
        api.do_autocmd(
            &["User"],
            lua.create_table_from([("pattern", "CompleetReinstate")])?,
        )?;
        api.do_autocmd(
            &["User"],
            lua.create_table_from([("pattern", "CompleetTryAttach")])?,
        )
    })?;

    nvim.schedule(trigger_try_attach)?;

    api.notify(
        "[nvim-compleet] Started completion in all buffers",
        LogLevel::Info,
    )?;

    Ok(())
}

fn attach_current_buffer(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let nvim = Neovim::new(lua)?;
    let api = &nvim.api;

    let bufnr = api.get_current_buf()?;

    if state.attached_buffers.contains(&bufnr) {
        api.notify(
            "[nvim-compleet] Completion is already on in this buffer",
            LogLevel::Error,
        )?;
        return Ok(());
    }

    // If this buffer was queued to be detached from buffer update events (the
    // ones setup by `nvim_buf_attach`, not autocmds) now it no longer needs
    // to.
    if state.buffers_to_be_detached.contains(&bufnr) {
        state.buffers_to_be_detached.retain(|&b| b != bufnr);
    }

    // Trigger a user autocmd to try to attach to this buffer. We don't add the
    // buffer to the `attached_buffers` vector here. That'll be done in
    // `try_buf_attach` once we successfully attach to the buffer.
    //
    // Also, we can't trigger the autocmd right here or the
    // `autocmds::try_buf_attach` function would be called, causing the global
    // state's mutex to deadlock. Instead we schedule it for a later time in
    // neovim's event loop via `vim.schedule`.
    let trigger_try_attach = lua.create_function(move |lua, ()| {
        let api = Neovim::new(lua)?.api;
        api.do_autocmd(
            &["User"],
            lua.create_table_from([("pattern", "CompleetTryAttach")])?,
        )
    })?;

    nvim.schedule(trigger_try_attach)?;

    // TODO: only display this once we've successfully attached to the
    // buffer.
    api.notify(
        &format!("[nvim-compleet] Started completion in buffer {bufnr}"),
        LogLevel::Info,
    )?;

    Ok(())
}
