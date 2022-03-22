use mlua::prelude::{Lua, LuaFunction, LuaResult};
use neovim::{api::LogLevel, Neovim};

use crate::state::State;

/// Executed on every `BufEnter` event and by the `CompleetStart{!}` user
/// command.
pub fn try_buf_attach(
    lua: &Lua,
    state: &mut State,
    on_bytes: LuaFunction,
    update_ui: LuaFunction,
    cleanup_ui: LuaFunction,
) -> LuaResult<()> {
    let api = Neovim::new(lua)?.api;

    let bufnr = api.get_current_buf()?;

    // Don't attach if:
    //
    // 1. we've already attached;
    //
    // 2. the buffer has the `modifiable` option turned off. This should catch
    //    a large number of buffers we'd like to ignore like netwr, startify,
    //    terminal buffers, help buffers, etc.
    if state.attached_buffers.contains(&bufnr)
        || !api.buf_get_option::<bool>(0, "modifiable")?
    {
        return Ok(());
    }

    let opts = lua.create_table_from([("on_bytes", on_bytes)])?;

    if api.buf_attach(0, false, opts)? {
        state.attached_buffers.push(bufnr);

        // We only add two buffer-local autocommands to update and cleanup the
        // ui once we've successfully attached to the buffer.

        let mut buffer_autocmd_ids = Vec::with_capacity(2);

        let opts = lua.create_table_with_capacity(0, 3)?;
        opts.set("group", state.augroup_id.expect("The augroup is set"))?;
        opts.set("buffer", bufnr)?;

        opts.set("callback", update_ui)?;
        buffer_autocmd_ids
            .push(api.create_autocmd(&["CursorMovedI"], opts.clone())?);

        opts.set("callback", cleanup_ui)?;
        buffer_autocmd_ids
            .push(api.create_autocmd(&["InsertLeave"], opts.clone())?);

        state
            .buffer_local_autocmds
            .insert(bufnr, buffer_autocmd_ids);

        // let ft = api.buf_get_option::<String>(bufnr, "filetype")?;
        // let bt = api.buf_get_option::<String>(bufnr, "buftype")?;
        // nvim.print(format!("{bufnr}, {ft}, {bt}"))?;
        // nvim.print(format!("{:?}", &state.attached_buffers))?;
    } else {
        api.notify(
            "[nvim-compleet] Couldn't attach to buffer",
            LogLevel::Error,
        )?;
    }

    Ok(())
}
