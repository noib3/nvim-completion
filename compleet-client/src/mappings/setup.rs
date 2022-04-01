use std::sync::Arc;

use mlua::prelude::{Lua, LuaResult};
use parking_lot::Mutex;

use crate::bindings::api;
use crate::state::State;

pub fn setup(lua: &Lua, state: &Arc<Mutex<State>>) -> LuaResult<()> {
    // Insert the currently hinted completion.
    let cloned = state.clone();
    let insert_hinted_completion = lua.create_function(move |lua, ()| {
        let locked = &mut cloned.lock();
        // if let Some(index) = locked.ui.completion_hint.hinted_index {
        //     super::insert_completion(lua, locked, index)?;
        // }
        Ok(())
    })?;

    // Insert the currently selected completion.
    let cloned = state.clone();
    let insert_selected_completion = lua.create_function(move |lua, ()| {
        let locked = &mut cloned.lock();
        // if let Some(index) = cloned.ui.completion_menu.selected_index {
        //     super::insert_completion(lua, cloned, index)?;
        // }
        Ok(())
    })?;

    // Select either the previous or next completion in the completion menu
    // based on the value of `step`.
    let cloned = state.clone();
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(lua, &mut cloned.lock(), step)
    })?;

    // Show the completion menu with all the currently available completion
    // candidates.
    let cloned = state.clone();
    let show_completions = lua.create_function(move |lua, ()| {
        super::show_completions(lua, &mut cloned.lock())
    })?;

    let opts = lua.create_table_from([("silent", true)])?;

    opts.set("callback", insert_hinted_completion)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-insert-hinted-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", insert_selected_completion)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-insert-selected-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", select_completion.bind(1)?)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-next-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", select_completion.bind(-1)?)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-prev-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", show_completions)?;
    api::set_keymap(lua, "i", "<Plug>(compleet-show-completions)", "", opts)?;

    Ok(())
}
