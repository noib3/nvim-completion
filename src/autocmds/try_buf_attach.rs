use mlua::prelude::{Lua, LuaFunction, LuaResult};
use neovim::{api::LogLevel, Neovim};

use crate::completion::Sources;
use crate::state::State;

/// Executed on every `BufEnter` event and by the `CompleetStart{!}` user
/// command.
pub fn try_buf_attach(
    lua: &Lua,
    state: &mut State,
    insert_leave: LuaFunction,
    cursor_moved_i: LuaFunction,
    on_bytes: LuaFunction,
) -> LuaResult<()> {
    let api = Neovim::new(lua)?.api;

    let bufnr = api.get_current_buf()?;

    // Collect all the completion sources that want to attach to the current
    // buffer.
    let sources = state
        .settings
        .sources
        .iter()
        .filter(|&s| s.attach(&api, bufnr).unwrap_or(false))
        .map(|s| s.clone())
        .collect::<Sources>();

    // Don't attach if:
    //
    // 1. we've already attached;
    //
    // 2. the buffer has the `modifiable` option turned off. This should catch
    //    a large number of buffers we'd like to ignore like netwr, startify,
    //    terminal buffers, help buffers, etc.
    //
    // 3. there are no compatible sources for the current buffer;
    if state.attached_buffers.contains(&bufnr)
        || !api.buf_get_option::<bool>(0, "modifiable")?
        || sources.is_empty()
    {
        return Ok(());
    }

    let opts = lua.create_table_from([("on_bytes", on_bytes)])?;

    if api.buf_attach(0, false, opts)? {
        state.attached_buffers.push(bufnr);

        let mut buffer_autocmd_ids = Vec::with_capacity(2);

        let opts = lua.create_table_with_capacity(0, 3)?;
        opts.set("group", state.augroup_id.expect("The augroup is set"))?;
        opts.set("buffer", bufnr)?;

        opts.set("callback", cursor_moved_i)?;
        buffer_autocmd_ids
            .push(api.create_autocmd(&["CursorMovedI"], opts.clone())?);

        opts.set("callback", insert_leave)?;
        buffer_autocmd_ids.push(api.create_autocmd(&["InsertLeave"], opts)?);

        state
            .buffer_local_autocmds
            .insert(bufnr, buffer_autocmd_ids);

        if state.sources.get(&bufnr).is_none() {
            state.sources.insert(bufnr, sources);
        }

        #[cfg(debug)]
        {
            let nvim = Neovim::new(lua)?;
            let ft = api.buf_get_option::<String>(bufnr, "filetype")?;
            let bt = api.buf_get_option::<String>(bufnr, "buftype")?;
            nvim.print(format!("{bufnr}, {ft}, {bt}"))?;
            nvim.print(format!("{:?}", &state.attached_buffers))?;
        }
    } else {
        api.notify(
            "[nvim-compleet] Couldn't attach to buffer",
            LogLevel::Error,
        )?;
    }

    Ok(())
}
