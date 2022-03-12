use mlua::{Lua, Result};
use neovim::Keymap;
use std::sync::{Arc, Mutex};

use crate::api;
use crate::state::State;

pub fn setup(
    lua: &Lua,
    keymap: &Keymap,
    state: &Arc<Mutex<State>>,
) -> Result<()> {
    // Insert the currently hinted completion.
    let _state = state.clone();
    let insert_hinted_completion = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        if let Some(index) = _state.ui.completion_hint.hinted_index {
            api::insert_completion(lua, _state, index)?;
        }
        Ok(())
    })?;

    // Insert the currently selected completion.
    let _state = state.clone();
    let insert_selected_completion = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        if let Some(index) = _state.ui.completion_menu.selected_completion {
            api::insert_completion(lua, _state, index)?;
        }
        Ok(())
    })?;

    // Select either the previous or next completion in the completion menu
    // based on the value of `step`.
    let _state = state.clone();
    let select_completion = lua.create_function(move |lua, step| {
        api::select_completion(lua, &mut _state.lock().unwrap(), step)
    })?;

    // Show the completion menu with all the currently available completion
    // candidates.
    let _state = state.clone();
    let show_completions = lua.create_function(move |lua, ()| {
        api::show_completions(lua, &mut _state.lock().unwrap())
    })?;

    let opts = lua.create_table_from([("silent", true)])?;

    keymap.set(
        "i",
        "<Plug>(compleet-insert-hinted-completion)",
        insert_hinted_completion,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-insert-selected-completion)",
        insert_selected_completion,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-next-completion)",
        select_completion.bind(1)?,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-prev-completion)",
        select_completion.bind(-1)?,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-show-completions)",
        show_completions,
        Some(opts.clone()),
    )?;

    Ok(())
}
