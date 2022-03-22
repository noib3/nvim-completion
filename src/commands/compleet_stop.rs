use mlua::prelude::LuaResult;
use neovim::api::{Api, LogLevel};

use crate::State;

/// Executed by the `CompleetStop` user command.
pub fn compleet_stop(
    api: &Api,
    state: &mut State,
    bang: bool,
) -> LuaResult<()> {
    // The `CompleetStop!` command detaches all the buffers, while
    // `CompleetStop` only detaches the current buffer. To detach a buffer we
    // need to return `true` the next time the `on_bytes` function is
    // called.
    match bang {
        true => detach_all_buffers(api, state),
        false => detach_current_buffer(api, state),
    }
}

fn detach_all_buffers(api: &Api, state: &mut State) -> LuaResult<()> {
    if let Some(id) = state.bufenter_autocmd_id {
        // Delete the autocmd for the `BufEnter` event.
        api.del_autocmd(id)?;

        // Delete all the buffer-local autocmds for all attached buffer.
        for bufnr in state.attached_buffers.iter() {
            for id in state.buffer_local_autocmds.get(bufnr).expect(
                "If the buffer was attached it had some buffer-local autocmds",
            ) {
                api.del_autocmd(*id)?;
            }
        }

        state.bufenter_autocmd_id = None;

        // Move all the buffer numbers from the `attached_buffers` vector to
        // `buffers_to_be_detached`.
        state.buffers_to_be_detached =
            state.attached_buffers.drain(..).collect();

        // Cleanup the UI in case the user has somehow executed
        // `CompleetStop!` without exiting insert mode (for example via an
        // autocmd. Unlikely but possible).
        state.ui.cleanup(api)?;

        api.notify(
            "[nvim-compleet] Stopped completion in all buffers",
            LogLevel::Info,
        )?;
    } else {
        api.notify(
            "[nvim-compleet] Completion is already off",
            LogLevel::Error,
        )?;
    }

    Ok(())
}

fn detach_current_buffer(api: &Api, state: &mut State) -> LuaResult<()> {
    let bufnr = api.get_current_buf()?;

    if !state.attached_buffers.contains(&bufnr) {
        api.notify(
            "[nvim-compleet] Completion is already off in this buffer",
            LogLevel::Error,
        )?;
        return Ok(());
    }

    state.attached_buffers.retain(|&b| b != bufnr);
    state.buffers_to_be_detached.push(bufnr);

    state.ui.cleanup(api)?;

    // Delete all the buffer-local autocmds we had set for this buffer.
    for autocmd_id in state
        .buffer_local_autocmds
        .get(&bufnr)
        .expect("If the buffer was attached it had some buffer-local autocmds")
    {
        api.del_autocmd(*autocmd_id)?;
    }

    api.notify(
        &format!("[nvim-compleet] Stopped completion for buffer {bufnr}"),
        LogLevel::Info,
    )?;
    Ok(())
}
