use mlua::{Lua, Result};
use neovim::{LogLevel, Neovim};
use std::sync::{Arc, Mutex};

use crate::completion;
use crate::state::State;

pub fn maybe_attach(lua: &Lua, state: &Arc<Mutex<State>>) -> Result<()> {
    let nvim = Neovim::new(lua)?;

    // If the buffer has the `modifiable` option turned off we don't attach.
    // This catches a large number of buffers we'd like to ignore like: netwr,
    // startify, terminal buffers, help buffers, etc.
    if !nvim.api.buf_get_option::<bool>(0, "modifiable")? {
        return Ok(());
    }

    let _state = state.clone();
    let bytes_changed = lua.create_function_mut(
        move |lua,
              (
            _,
            _,
            _,
            start_row,
            start_col,
            _,
            old_end_row,
            _,
            old_end_bytelen,
            new_end_row,
            _,
            new_end_bytelen,
        ): (
            String,
            usize,
            usize,
            _,
            _,
            usize,
            _,
            usize,
            _,
            _,
            usize,
            _,
        )| {
            completion::bytes_changed(
                lua,
                &mut _state.lock().unwrap(),
                start_row,
                start_col,
                old_end_row,
                old_end_bytelen,
                new_end_row,
                new_end_bytelen,
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
