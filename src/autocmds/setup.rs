use mlua::{Function, Lua, Result, Table};
use neovim::Api;
use std::sync::{Arc, Mutex};

use crate::api;
use crate::state::State;

const COMPLEET_AUGROUP_NAME: &'static str = "Compleet";

pub fn setup(lua: &Lua, api: &Api, state: &Arc<Mutex<State>>) -> Result<()> {
    let _state = state.clone();
    let cleanup = lua.create_function(move |lua, ()| {
        api::cleanup_ui(lua, &mut _state.lock().unwrap().ui)
    })?;

    let _state = state.clone();
    let maybe_show_hint = lua.create_function(move |lua, ()| {
        api::maybe_show_hint(lua, &mut _state.lock().unwrap())
    })?;

    let _state = state.clone();
    let text_changed = lua.create_function(move |lua, ()| {
        api::text_changed(lua, &mut _state.lock().unwrap())
    })?;

    let opts = lua.create_table_from([("clear", true)])?;
    let _group_id = api.create_augroup(COMPLEET_AUGROUP_NAME, opts)?;

    let opts_w_callback = |callback: Function| -> Result<Table> {
        let opts = lua.create_table_with_capacity(0, 2)?;
        // TODO: why doesn't it work if I use the group id returned by
        // `create_augroup` here instead of the name?
        // opts.set("group", group_id)?;
        opts.set("group", COMPLEET_AUGROUP_NAME)?;
        opts.set("callback", callback)?;
        Ok(opts)
    };

    api.create_autocmd(
        &["CursorMovedI", "InsertLeave"],
        opts_w_callback(cleanup)?,
    )?;
    api.create_autocmd(
        &["CursorMovedI", "InsertEnter"],
        opts_w_callback(maybe_show_hint)?,
    )?;
    api.create_autocmd(&["TextChangedI"], opts_w_callback(text_changed)?)?;

    Ok(())
}
