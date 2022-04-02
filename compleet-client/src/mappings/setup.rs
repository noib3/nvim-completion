use std::{cell::RefCell, rc::Rc};

use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;
use crate::state::State;

pub fn setup(lua: &Lua, state: &Rc<RefCell<State>>) -> LuaResult<()> {
    // Insert the currently hinted completion.
    let cloned = state.clone();
    let insert_hinted_completion = lua.create_function(move |lua, ()| {
        let borrowed = &mut cloned.borrow_mut();
        if let Some(index) = borrowed.ui.as_ref().unwrap().hint.hinted_index {
            super::insert_completion(lua, borrowed, index)?;
        }
        Ok(())
    })?;

    // Insert the currently selected completion.
    let cloned = state.clone();
    let insert_selected_completion = lua.create_function(move |lua, ()| {
        let borrowed = &mut cloned.borrow_mut();
        if let Some(index) = borrowed.ui.as_ref().unwrap().menu.selected_index
        {
            super::insert_completion(lua, borrowed, index)?;
        }
        Ok(())
    })?;

    // Select either the previous or next completion in the completion menu
    // based on the value of `step`.
    let cloned = state.clone();
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(lua, &mut cloned.borrow_mut(), step)
    })?;

    // Show the completion menu with all the currently available completion
    // candidates.
    let cloned = state.clone();
    let show_completions = lua.create_function(move |lua, ()| {
        super::show_completions(lua, &mut cloned.borrow_mut())
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
