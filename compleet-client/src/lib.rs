#![feature(iter_intersperse)]
mod autocmds;
mod bindings;
mod channel;
mod commands;
mod constants;
mod hlgroups;
mod mappings;
mod settings;
mod setup;
mod state;
mod ui;
mod utils;

use std::{cell::RefCell, rc::Rc};

use mlua::{prelude::LuaResult, Lua, Table};

use crate::state::State;

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
    let state = Rc::new(RefCell::new(State::default()));

    let cloned = state.clone();
    let has_completions = lua.create_function(move |_lua, ()| {
        // TODO
        let _ = cloned.borrow_mut();
        Ok(false)
    })?;

    let cloned = state.clone();
    let is_completion_selected = lua.create_function(move |_lua, ()| {
        Ok(cloned.borrow().ui.menu.selected_index.is_some())
    })?;

    let cloned = state.clone();
    let is_hint_visible = lua.create_function(move |_lua, ()| {
        Ok(cloned.borrow().ui.hint.is_visible)
    })?;

    let cloned = state.clone();
    let is_menu_open = lua.create_function(move |_lua, ()| {
        Ok(cloned.borrow().ui.menu.floater.is_open())
    })?;

    let setup = lua.create_function(move |lua, preferences| {
        setup::setup(lua, &state, preferences)
    })?;

    lua.create_table_from([
        ("has_completions", has_completions),
        ("is_completion_selected", is_completion_selected),
        ("is_hint_visible", is_hint_visible),
        ("is_menu_open", is_menu_open),
        ("setup", setup),
    ])
}
