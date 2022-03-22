use mlua::prelude::{Lua, LuaResult};
use neovim::{Api, Neovim};
use std::sync::{Arc, Mutex};

use crate::completion;
use crate::state::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<(u32, Box<dyn 'static + Fn(&Lua, ()) -> LuaResult<()>>)> {
    let _state = state.clone();
    let cleanup_ui = move |lua: &Lua, ()| {
        let api = Neovim::new(&lua)?.api;
        let ui = &mut _state.lock().unwrap().ui;
        ui.cleanup(&api)
    };

    let _state = state.clone();
    let update_ui = move |lua: &Lua, ()| {
        let api = Neovim::new(lua)?.api;
        let state = &mut *_state.lock().unwrap();
        state.ui.update(
            lua,
            &api,
            &state.completions,
            &state.cursor,
            &state.settings,
        )
    };

    let _state = state.clone();
    let on_bytes =
        move |lua: &Lua,
              (
            _,
            bufnr,
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
        ): (String, _, u32, _, _, u32, _, u32, _, _, u32, _)| {
            completion::on_bytes(
                lua,
                &mut _state.lock().unwrap(),
                bufnr,
                start_row,
                start_col,
                rows_deleted,
                bytes_deleted,
                rows_added,
                bytes_added,
            )
        };

    let _state = state.clone();
    let try_buf_attach = move |lua: &Lua, ()| {
        let on_bytes = lua.create_function(on_bytes.clone())?;
        let update_ui = lua.create_function(update_ui.clone())?;
        let cleanup_ui = lua.create_function(cleanup_ui.clone())?;

        super::try_buf_attach(
            lua,
            &mut _state.lock().unwrap(),
            on_bytes,
            update_ui,
            cleanup_ui,
        )
    };

    // Create the `Compleet` augroup which will hold all our autocmds.
    let opts = lua.create_table_from([("clear", true)])?;
    let augroup_id = api.create_augroup("Compleet", opts)?;

    let opts = lua.create_table_with_capacity(0, 2)?;
    opts.set("group", augroup_id)?;
    opts.set("callback", lua.create_function(try_buf_attach.clone())?)?;
    api.create_autocmd(&["BufEnter"], opts.clone())?;

    Ok((augroup_id, Box::new(try_buf_attach.clone())))
}
