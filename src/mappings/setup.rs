use mlua::prelude::{Lua, LuaResult};
use neovim::Api;
use std::sync::{Arc, Mutex};

use crate::state::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<()> {
    // Insert the currently hinted completion.
    let _state = state.clone();
    let insert_hinted_completion = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        if let Some(index) = _state.ui.completion_hint.hinted_index {
            super::insert_completion(lua, _state, index)?;
        }
        Ok(())
    })?;

    // Insert the currently selected completion.
    let _state = state.clone();
    let insert_selected_completion = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        if let Some(index) = _state.ui.completion_menu.selected_index {
            super::insert_completion(lua, _state, index)?;
        }
        Ok(())
    })?;

    // Select either the previous or next completion in the completion menu
    // based on the value of `step`.
    let _state = state.clone();
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(lua, &mut _state.lock().unwrap(), step)
    })?;

    // Show the completion menu with all the currently available completion
    // candidates.
    let _state = state.clone();
    let show_completions = lua.create_function(move |lua, ()| {
        super::show_completions(lua, &mut _state.lock().unwrap())
    })?;

    let opts = lua.create_table_from([("silent", true)])?;

    opts.set("callback", insert_hinted_completion)?;
    api.set_keymap(
        "i",
        "<Plug>(compleet-insert-hinted-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", insert_selected_completion)?;
    api.set_keymap(
        "i",
        "<Plug>(compleet-insert-selected-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", select_completion.bind(1)?)?;
    api.set_keymap("i", "<Plug>(compleet-next-completion)", "", opts.clone())?;

    opts.set("callback", select_completion.bind(-1)?)?;
    api.set_keymap("i", "<Plug>(compleet-prev-completion)", "", opts.clone())?;

    opts.set("callback", show_completions)?;
    api.set_keymap(
        "i",
        "<Plug>(compleet-show-completions)",
        "",
        opts.clone(),
    )?;

    Ok(())
}
