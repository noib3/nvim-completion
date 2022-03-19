use mlua::{prelude::LuaResult, Lua, Table};
use neovim::Neovim;
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

// BUGs
//
// 1. Completion details is in the wrong column when completion is selected.

// TODOs: On Hold
//
// 1. Show scroll indicator if number of completions is bigger than the
//    completion menu's max height. This needs floating windows to support
//    scrollbars. See `:h api-floatwin`.

// TODOs
//
// 0. Trigger a `autocmds::try_buf_attach` on `require('compleet').setup`.
//
// 1. Right now everything is sync and we're blocking on every single event we
//    listen to. This will be a problem when we start dealing with possibly
//    thousands of completion results from LSPs. Can we leverage async on the
//    Rust end w/ Tokyo? Also look into `:h vim.loop` and `:h
//    lua-loop-threading`.
//
// 2. Add padding to both details and completion windows instead of relying on
//    spaces?
//
// ** 3. Add option to set the completion window at the start of the completion
// instead of the cursor.
//
// 4. Use serde to deserialize settings into struct.
//
// 5. Make `lipsum` an actual source.
//
// 6. `Compleet{Start, Stop}!` to attach and detach from all buffers, with
//    version w/o ! only attaching to/detaching from the current buffer.
//
// 7. `CursorMovedI` and `InsertLeave` should be <buffer> local autocmds only
//    for attached buffers.
//
// 8. Provide fallback for options that don't exist (e.g. `did you mean y?`).
//
// 9. If value is not valid display what the user passed (e.g. `expected a
//    boolean, found '"true"'`).
//
// 10. Return all wrong options at once instead of stopping at the first one.
//
// 11. Add nvim integration tests (lua?).

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
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

    let exports = lua.create_table_with_capacity(0, 5)?;
    exports.set("has_completions", has_completions)?;
    exports.set("is_completion_selected", is_completion_selected)?;
    exports.set("is_hint_visible", is_hint_visible)?;
    exports.set("is_menu_visible", is_menu_visible)?;
    exports.set("setup", setup)?;
    Ok(exports)
}
