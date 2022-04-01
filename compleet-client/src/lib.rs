use std::rc::Rc;

use mlua::{prelude::LuaResult, Lua, Table};
use parking_lot::Mutex;
use state::State;

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

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
    let state = Rc::new(Mutex::new(State::new(lua)?));

    let clone = state.clone();
    let has_completions = lua.create_function(move |_lua, ()| {
        let _ = &mut clone.lock();
        Ok(false)
    })?;

    let clone = state.clone();
    let is_completion_selected = lua.create_function(move |_lua, ()| {
        let _ = &mut clone.lock();
        Ok(false)
    })?;

    let clone = state.clone();
    let is_hint_visible = lua.create_function(move |_lua, ()| {
        let _ = &mut clone.lock();
        Ok(false)
    })?;

    let clone = state.clone();
    let is_menu_visible = lua.create_function(move |_lua, ()| {
        let _ = &mut clone.lock();
        Ok(false)
    })?;

    let setup = lua.create_function(move |lua, preferences| {
        setup::setup(lua, &state, preferences)
    })?;

    lua.create_table_from([
        ("has_completions", has_completions),
        ("is_completion_selected", is_completion_selected),
        ("is_hint_visible", is_hint_visible),
        ("is_menu_visible", is_menu_visible),
        ("setup", setup),
    ])
}
