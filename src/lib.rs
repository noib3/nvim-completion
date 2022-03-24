use mlua::{prelude::LuaResult, Lua, Table};
use neovim::Neovim;
use std::panic;
use std::sync::{Arc, Mutex};

mod api;
mod autocmds;
mod commands;
mod completion;
mod hlgroups;
mod mappings;
mod settings;
mod state;
mod ui;

use state::State;

/*
BUGs:
1. (ui) menu's position doesn't update when the signcolumn changes;

TODOs: On Hold

1. Show scroll indicator if number of completions is bigger than the completion
   menu's max height. This needs floating windows to support scrollbars. See
   `:h api-floatwin`.

TODOs

1. Right now everything is sync. This will be a problem when we start dealing
   with possibly thousands of completion results from LSPs. Can we leverage
   async on the Rust end w/ Tokyo? Also look into `:h vim.loop` and `:h
   lua-loop-threading`.

2. border should have different defaults for menu and details;

3. border chars should be either 0 or 1 chars

4. details window should shift when scrolling options instead of redrawing;

5. add doc comments and solve as many TODOs as possible.
*/

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
    // Because the plugin runs in the main thread, panics will take down the
    // whole neovim process. We can't do a lot except relaying the panic infos.
    panic::set_hook(Box::new(|infos| {
        eprintln!(
            "[nvim-compleet] {infos}. \
             Please open a new issue at \
             'https://github.com/noib3/nvim-compleet/issues'."
        );
        std::process::exit(1);
    }));

    let api = Neovim::new(lua)?.api;
    let state = Arc::new(Mutex::new(State::new(&api)?));

    let _state = state.clone();
    let has_completions = lua.create_function(move |lua, ()| {
        api::has_completions(lua, &mut _state.lock().unwrap())
    })?;

    let _state = state.clone();
    let is_completion_selected = lua.create_function(move |_, ()| {
        Ok(_state.lock().unwrap().ui.completion_menu.is_item_selected())
    })?;

    let _state = state.clone();
    let is_hint_visible = lua.create_function(move |_, ()| {
        Ok(_state.lock().unwrap().ui.completion_hint.is_visible())
    })?;

    let _state = state.clone();
    let is_menu_visible = lua.create_function(move |_, ()| {
        Ok(_state.lock().unwrap().ui.completion_menu.is_visible())
    })?;

    let setup = lua.create_function(move |lua, preferences| {
        api::setup(lua, &state, preferences)
    })?;

    Ok(lua.create_table_from([
        ("has_completions", has_completions),
        ("is_completion_selected", is_completion_selected),
        ("is_hint_visible", is_hint_visible),
        ("is_menu_visible", is_menu_visible),
        ("setup", setup),
    ])?)
}
