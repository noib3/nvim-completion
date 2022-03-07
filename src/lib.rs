use mlua::{Lua, Result, Table};
use std::sync::Arc;

mod api;
mod completion;
mod config;
mod nvim;
mod state;
mod ui;

use nvim::Nvim;
use state::State;

#[mlua::lua_module]
fn compleet(lua: &Lua) -> Result<Table> {
    // TODO: right now everything is sync and we're blocking on every single
    // event we listen to. This will be a problem when we start dealing with
    // possibly thousands of completion results from LSPs.
    // Can we leverage async on the Rust end w/ tokyo? Also look into `:h
    // vim.loop` and `:h lua-loop-threading`.

    // TODO: bazooka then ALT-BS stops at baz?

    let nvim = Nvim::new(lua)?;
    let state = State::new(&nvim)?;

    let completion_state = Arc::clone(&state.completion);
    let has_completions = lua.create_function(move |lua, ()| {
        api::has_completions(lua, &mut completion_state.lock().unwrap())
    })?;

    let ui_state = Arc::clone(&state.ui);
    let is_completion_selected = lua.create_function(move |_, ()| {
        Ok(api::is_completion_item_selected(&ui_state.lock().unwrap()))
    })?;

    let ui_state = Arc::clone(&state.ui);
    let is_hint_visible = lua.create_function(move |_, ()| {
        Ok(api::is_completion_hint_visible(&ui_state.lock().unwrap()))
    })?;

    let ui_state = Arc::clone(&state.ui);
    let is_menu_visible = lua.create_function(move |_, ()| {
        Ok(api::is_completion_menu_visible(&ui_state.lock().unwrap()))
    })?;

    let setup = lua
        .create_function(move |lua, config| api::setup(lua, &state, config))?;

    let exports = lua.create_table_with_capacity(0, 5)?;
    exports.set("has_completions", has_completions)?;
    exports.set("is_completion_selected", is_completion_selected)?;
    exports.set("is_hint_visible", is_hint_visible)?;
    exports.set("is_menu_visible", is_menu_visible)?;
    exports.set("setup", setup)?;
    Ok(exports)
}
