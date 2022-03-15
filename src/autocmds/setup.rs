use mlua::{Lua, Result};
use neovim::Api;
use std::sync::{Arc, Mutex};

use crate::state::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> Result<usize> {
    let _state = state.clone();
    let cleanup = lua.create_function(move |lua, ()| {
        super::cleanup_ui(lua, &mut _state.lock().unwrap().ui)
    })?;

    let _state = state.clone();
    let maybe_show_hint = lua.create_function(move |lua, ()| {
        super::maybe_show_hint(lua, &mut _state.lock().unwrap())
    })?;

    let _state = state.clone();
    let text_changed = lua.create_function(move |lua, ()| {
        super::text_changed(lua, &mut _state.lock().unwrap())
    })?;

    let opts = lua.create_table_from([("clear", true)])?;
    let augroup_id = api.create_augroup("Compleet", opts)?;

    let opts = lua.create_table_from([("group", augroup_id)])?;

    opts.set("callback", cleanup)?;
    api.create_autocmd(&["CursorMovedI", "InsertLeave"], opts.clone())?;

    opts.set("callback", maybe_show_hint)?;
    api.create_autocmd(&["CursorMovedI", "InsertEnter"], opts.clone())?;

    opts.set("callback", text_changed)?;
    api.create_autocmd(&["TextChangedI"], opts.clone())?;

    Ok(augroup_id)
}