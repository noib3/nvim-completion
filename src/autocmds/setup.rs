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
    let maybe_show_completions = lua.create_function(move |lua, ()| {
        super::update_ui(lua, &mut _state.lock().unwrap())
    })?;

    // let _state = state.clone();
    // let maybe_show_hint = lua.create_function(move |lua, ()| {
    //     super::maybe_show_hint(lua, &mut _state.lock().unwrap())
    // })?;

    // let _state = state.clone();
    // let text_changed = lua.create_function(move |lua, ()| {
    //     super::text_changed(lua, &mut _state.lock().unwrap())
    // })?;

    let _state = state.clone();
    let try_buf_attach = lua
        .create_function(move |lua, ()| super::try_buf_attach(lua, &_state))?;

    let opts = lua.create_table_from([("clear", true)])?;
    let augroup_id = api.create_augroup("Compleet", opts)?;

    let opts = lua.create_table_from([("group", augroup_id)])?;

    opts.set("callback", cleanup)?;
    api.create_autocmd(&["InsertLeave"], opts.clone())?;

    opts.set("callback", maybe_show_completions)?;
    api.create_autocmd(&["CursorMovedI"], opts.clone())?;

    // opts.set("callback", maybe_show_hint)?;
    // api.create_autocmd(&["CursorMovedI"], opts.clone())?;

    // opts.set("callback", text_changed)?;
    // api.create_autocmd(&["TextChangedI"], opts.clone())?;

    opts.set("callback", try_buf_attach)?;
    api.create_autocmd(&["BufEnter"], opts.clone())?;

    Ok(augroup_id)
}
