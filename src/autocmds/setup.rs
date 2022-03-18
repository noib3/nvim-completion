use mlua::{Lua, Result};
use neovim::{Api, Neovim};
use std::sync::{Arc, Mutex};

use crate::state::State;

pub fn setup(lua: &Lua, api: &Api, state: &Arc<Mutex<State>>) -> Result<u32> {
    let _state = state.clone();
    let cleanup_ui = lua.create_function(move |lua, ()| {
        let api = Neovim::new(&lua)?.api;
        let ui = &mut _state.lock().unwrap().ui;
        ui.cleanup(&api)
    })?;

    let _state = state.clone();
    let update_ui = lua.create_function(move |lua, ()| {
        let api = Neovim::new(&lua)?.api;
        let state = &mut *_state.lock().unwrap();
        state
            .ui
            .update(lua, &api, &state.cursor, &state.completions)
    })?;

    let _state = state.clone();
    let try_buf_attach = lua
        .create_function(move |lua, ()| super::try_buf_attach(lua, &_state))?;

    let opts = lua.create_table_from([("clear", true)])?;
    let augroup_id = api.create_augroup("Compleet", opts)?;

    let opts = lua.create_table_from([("group", augroup_id)])?;

    opts.set("callback", cleanup_ui)?;
    api.create_autocmd(&["InsertLeave"], opts.clone())?;

    opts.set("callback", update_ui)?;
    api.create_autocmd(&["CursorMovedI"], opts.clone())?;

    opts.set("callback", try_buf_attach)?;
    api.create_autocmd(&["BufEnter"], opts.clone())?;

    Ok(augroup_id)
}
