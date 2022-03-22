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
    let on_bytes = lua.create_function(
        move |lua,
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
        },
    )?;

    // Create the `Compleet` augroup which will hold all our autocmds.
    let opts = lua.create_table_from([("clear", true)])?;
    let augroup_id = api.create_augroup("Compleet", opts)?;

    let _state = state.clone();
    let try_buf_attach = lua
        .create_function(
            move |lua,
                  (on_bytes, update_ui, cleanup_ui): (
                LuaFunction,
                LuaFunction,
                LuaFunction,
            )| {
                super::try_buf_attach(
                    lua,
                    &mut _state.lock().unwrap(),
                    on_bytes,
                    augroup_id,
                    update_ui,
                    cleanup_ui,
                )
            },
        )?
        .bind(on_bytes)?
        .bind(update_ui)?
        .bind(cleanup_ui)?;

    let opts = lua.create_table_with_capacity(0, 3)?;
    opts.set("group", augroup_id)?;
    opts.set("callback", try_buf_attach.clone())?;

    let bufenter_autocmd_id =
        api.create_autocmd(&["BufEnter"], opts.clone())?;

    // TODO: docs
    opts.set("pattern", "CompleetTryAttach")?;
    api.create_autocmd(&["User"], opts.clone())?;

    // TODO: docs
    let _state = state.clone();
    let reinstate_bufenter = lua
        .create_function(move |lua, try_buf_attach: LuaFunction| {
            let opts = lua.create_table_with_capacity(0, 2)?;
            opts.set("group", augroup_id)?;
            opts.set("callback", try_buf_attach)?;

            let state = &mut _state.lock().unwrap();

            let api = Neovim::new(lua)?.api;
            state.bufenter_autocmd_id =
                Some(api.create_autocmd(&["BufEnter"], opts)?);

            Ok(())
        })?
        .bind(try_buf_attach)?;

    opts.set("callback", reinstate_bufenter)?;
    opts.set("pattern", "CompleetReinstate")?;
    api.create_autocmd(&["User"], opts.clone())?;

    Ok(bufenter_autocmd_id)
}
