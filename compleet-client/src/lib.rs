mod autocmds;
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

use bindings::lsp;
use mlua::{prelude::LuaResult, Lua, Table};

use crate::{settings::Settings, state::State};

fn lsp_client_capabilities<'lua>(
    lua: &'lua Lua,
    _settings: &Settings,
) -> LuaResult<Table<'lua>> {
    let capabilities = lsp::make_client_capabilities(lua)?;

    let completion = capabilities
        .get::<_, Table>("textDocument")?
        .get::<_, Table>("completion")?;

    completion.set("dynamicRegistration", true)?;
    completion.set("contextSupport", true)?;

    let completion_item = completion.get::<_, Table>("completionItem")?;

    // TODO: check if lsp has any snippets engines enabled before setting this
    // to true.
    completion_item.set("snippetSupport", false)?;

    completion_item.set("deprecatedSupport", true)?;
    completion_item.set("insertReplaceSupport", true)?;
    completion_item.set("labelDetailsSupport", true)?;

    Ok(capabilities)
}

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
    let state = Rc::<RefCell<State>>::default();

    let cloned = state.clone();
    let lsp_client_capabilities = lua.create_function(move |lua, ()| {
        self::lsp_client_capabilities(lua, &cloned.borrow().settings)
    })?;

    let cloned = state.clone();
    let has_completions = lua.create_function(move |_lua, ()| {
        // TODO: implement this
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
        ("lsp_client_capabilities", lsp_client_capabilities),
        ("has_completions", has_completions),
        ("is_completion_selected", is_completion_selected),
        ("is_hint_visible", is_hint_visible),
        ("is_menu_open", is_menu_open),
        ("setup", setup),
    ])
}
