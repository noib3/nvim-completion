use mlua::{Lua, Result};
use neovim::{LogLevel, Neovim};
use std::sync::{Arc, Mutex};

use crate::completion;
use crate::state::State;

/// Executed on every `BufEnter` event.
pub fn try_buf_attach(lua: &Lua, state: &Arc<Mutex<State>>) -> Result<()> {
    let nvim = Neovim::new(lua)?;

    // If the buffer has the `modifiable` option turned off we don't attach.
    // This should catch a large number of buffers we'd like to ignore like
    // netwr, startify, terminal buffers, help buffers, etc.
    if !nvim.api.buf_get_option::<bool>(0, "modifiable")? {
        return Ok(());
    }

    let _state = state.clone();
    let bytes_changed = lua.create_function(
        move |lua,
              (
            _,
            _,
            _,
            start_row,
            start_col,
            _,
            rows_deleted,
            _,
            bytes_deleted,
            rows_added,
            _,
            bytes_added,
        ): (
            String,
            u32,
            u32,
            _,
            _,
            u32,
            _,
            u32,
            _,
            _,
            u32,
            _,
        )| {
            completion::bytes_changed(
                lua,
                &mut _state.lock().unwrap(),
                start_row,
                start_col,
                rows_deleted,
                bytes_deleted,
                rows_added,
                bytes_added,
            )
        },
    )?;

    let opts = lua.create_table_from([("on_bytes", bytes_changed)])?;

    if !nvim.api.buf_attach(0, false, opts)? {
        nvim.notify(
            "[nvim-compleet]: Couldn't attach to buffer.",
            LogLevel::Error,
            lua.create_table()?,
        )?;
    }

    Ok(())
}
