use mlua::prelude::{Lua, LuaFunction, LuaResult};
use neovim::{Api, Neovim};
use std::sync::{Arc, Mutex};

use crate::completion;
use crate::state::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<u32> {
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
        state.ui.update(
            lua,
            &api,
            &state.completions,
            &state.cursor,
            &state.settings,
        )
    })?;

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

    let _state = state.clone();
    let try_buf_attach =
        lua.create_function(move |lua, bytes_changed: LuaFunction| {
            super::try_buf_attach(
                lua,
                &mut _state.lock().unwrap(),
                bytes_changed,
            )
        })?;

    let opts = lua.create_table_from([("clear", true)])?;
    let augroup_id = api.create_augroup("Compleet", opts)?;

    let opts = lua.create_table_from([("group", augroup_id)])?;

    opts.set("callback", try_buf_attach.bind(bytes_changed)?)?;
    api.create_autocmd(&["BufEnter"], opts.clone())?;

    opts.set("callback", update_ui)?;
    api.create_autocmd(&["CursorMovedI"], opts.clone())?;

    opts.set("callback", cleanup_ui)?;
    api.create_autocmd(&["InsertLeave"], opts.clone())?;

    Ok(augroup_id)
}
