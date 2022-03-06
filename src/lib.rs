use mlua::{Lua, Result, Table};
use std::sync::Arc;

mod api;
mod completion;
mod config;
mod insertion;
mod nvim;
mod state;
mod ui;

use nvim::Nvim;
use state::State;

#[mlua::lua_module]
fn compleet(lua: &Lua) -> Result<Table> {
    // TODO: look into vim's event loop. Are we blocking on every single event
    // we listen to? Is there a way not to? Can we leverage async on the Rust
    // end in some way?
    // TODO: bazooka then ALT-BS stops at baz?

    let nvim = Nvim::new(lua)?;
    let state = State::new(&nvim)?;

    let ui_state = Arc::clone(&state.ui);
    let cursor_moved = lua.create_function(move |lua, ()| {
        api::cursor_moved(lua, &mut ui_state.lock().unwrap())
    })?;

    let ui_state = Arc::clone(&state.ui);
    let insert_left = lua.create_function(move |lua, ()| {
        api::insert_left(lua, &mut ui_state.lock().unwrap())
    })?;

    let completion_state = Arc::clone(&state.completion);
    let config = Arc::clone(&state.config);
    let ui_state = Arc::clone(&state.ui);
    let text_changed = lua.create_function(move |lua, ()| {
        api::text_changed(
            lua,
            &config.lock().unwrap(),
            &mut completion_state.lock().unwrap(),
            &mut ui_state.lock().unwrap(),
        )
    })?;

    let events = lua.create_table_with_capacity(0, 3)?;
    events.set("cursor_moved", cursor_moved)?;
    events.set("insert_left", insert_left)?;
    events.set("text_changed", text_changed)?;

    let completion_state = Arc::clone(&state.completion);
    let has_completions = lua.create_function(move |lua, ()| {
        api::has_completions(lua, &mut completion_state.lock().unwrap())
    })?;

    let ui_state = Arc::clone(&state.ui);
    let is_completion_item_selected = lua.create_function(move |_, ()| {
        Ok(api::is_completion_item_selected(&ui_state.lock().unwrap()))
    })?;

    let ui_state = Arc::clone(&state.ui);
    let is_completion_hint_visible = lua.create_function(move |_, ()| {
        Ok(api::is_completion_hint_visible(&ui_state.lock().unwrap()))
    })?;

    let ui_state = Arc::clone(&state.ui);
    let is_completion_menu_visible = lua.create_function(move |_, ()| {
        Ok(api::is_completion_menu_visible(&ui_state.lock().unwrap()))
    })?;

    let setup = lua
        .create_function(move |lua, config| api::setup(lua, &state, config))?;

    let exports = lua.create_table_with_capacity(0, 6)?;
    exports.set("__events", events)?;
    exports.set("has_completions", has_completions)?;
    exports.set("is_completion_selected", is_completion_item_selected)?;
    exports.set("is_hint_visible", is_completion_hint_visible)?;
    exports.set("is_menu_visible", is_completion_menu_visible)?;
    exports.set("setup", setup)?;
    Ok(exports)
}
