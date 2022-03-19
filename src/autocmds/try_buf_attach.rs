use mlua::prelude::{Lua, LuaFunction, LuaResult};
use neovim::{LogLevel, Neovim};

use crate::state::State;

/// Executed on every `BufEnter` event.
pub fn try_buf_attach(
    lua: &Lua,
    state: &mut State,
    bytes_changed: LuaFunction,
) -> LuaResult<()> {
    let nvim = Neovim::new(lua)?;
    let api = &nvim.api;

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

    let opts = lua.create_table_from([("on_bytes", bytes_changed)])?;

    if api.buf_attach(0, false, opts)? {
        state.attached_buffers.push(bufnr);
        // let ft = api.buf_get_option::<String>(bufnr, "filetype")?;
        // let bt = api.buf_get_option::<String>(bufnr, "buftype")?;
        // nvim.print(format!("{bufnr}, {ft}, {bt}"))?;
        // nvim.print(format!("{:?}", &state.attached_buffers))?;
    } else {
        nvim.notify(
            "[nvim-compleet]: Couldn't attach to buffer.",
            LogLevel::Error,
            lua.create_table()?,
        )?;
    }

    Ok(())
}
