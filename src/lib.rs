use mlua::{Lua, Result, Table};
use std::sync::{Arc, Mutex};

mod api;
mod completion;
mod nvim;
mod settings;
mod state;
mod ui;

use nvim::Nvim;
use state::State;

#[mlua::lua_module]
fn compleet(lua: &Lua) -> Result<Table> {
    // TODOs

    // 0. BUG: `this is a f<Super-Left><Super-Right>roo<Alt-BS>` -> foo is
    //    still visible.

    // 1. Color matching chars.

    // 2. Create custom highlight groups for `CompleetUnselectedItem`,
    // `CompleetSelectedItem`, `CompleetMatchingCharsSelected`,
    // `CompleetMatchingCharsUnselected`, or something like that.

    // 3. Show bottom indicator with something like `item 1 of 3`, and
    // some other message when no item is selected.

    // 4. Add config option to set the maximum height of the completion
    // menu.

    // 5. Scroll buffer to keep selected completion visible if number of
    // completions is bigger than the completion menu's max height.

    // 6. Handle geometry for completion menu, i.e. show it above the
    // current line if there's not enough space below it. Also handle
    // horizontal constraints.

    // 7. Implement details pane.

    // 8. Handle geometry for details pane.

    // 9. Move nvim to its own crate, call via `nvim.api`, `nvim.keymap`, etc.

    // 10. Make the core logic as neovim-agnostic as possible.

    // 11. Right now everything is sync and we're blocking on every single
    // event we listen to. This will be a problem when we start dealing with
    // possibly thousands of completion results from LSPs.
    //
    // Can we leverage async on the Rust end w/ Tokyo? Also look into `:h
    // vim.loop` and `:h lua-loop-threading`.

    let nvim = Nvim::new(lua)?;
    let state = Arc::new(Mutex::new(State::new(&nvim)?));

    let _state = Arc::clone(&state);
    let has_completions = lua.create_function(move |lua, ()| {
        api::has_completions(lua, &mut _state.lock().unwrap().completion)
    })?;

    let _state = Arc::clone(&state);
    let is_completion_selected = lua.create_function(move |_, ()| {
        Ok(_state.lock().unwrap().ui.completion_menu.is_item_selected())
    })?;

    let _state = Arc::clone(&state);
    let is_hint_visible = lua.create_function(move |_, ()| {
        Ok(_state.lock().unwrap().ui.completion_hint.is_visible())
    })?;

    let _state = Arc::clone(&state);
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
