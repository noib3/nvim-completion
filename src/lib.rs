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

/*
TODOs: On Hold

1. Show scroll indicator if number of completions is bigger than the completion
   menu's max height. This needs floating windows to support scrollbars. See
   `:h api-floatwin`.

TODOs

1. Trigger a `autocmds::try_buf_attach` on `require('compleet').setup`.

2. Right now everything is sync and we're blocking on every single event we
   listen to. This will be a problem when we start dealing with possibly
   thousands of completion results from LSPs. Can we leverage async on the Rust
   end w/ Tokyo? Also look into `:h vim.loop` and `:h lua-loop-threading`.

3. Make `lipsum` an actual source by creating `CompletionSource` trait. That
   trait has a `complete` function that takes some context and returns a
   `Vec<CompletionItem>`. Call lipsum by `lipsum.complete(&ctx)` or something.

4. `Compleet{Start, Stop}!` to attach and detach from all buffers, with version
   w/o ! only attaching to/detaching from the current buffer.

5. `CursorMovedI` and `InsertLeave` should be <buffer> local autocmds only for
   attached buffers.

6. Better error reporting for wrongly formed preferences, e.g.:

   * `Invalid option "foo" for `ui.menu.anchor`, valid options are "cursor"
   and "match"`;

   * `Wrong type `boolean` for `ui.menu.anchor`: valid options are "cursor"
   and "match"`;

   * `Invalid field `ui.foo`, valid fields are `ui.menu`, `ui.details` and
   `ui.hint`;

   * `Wrong type `boolean` for `ui.menu.anchor`: valid options are "cursor"
   and "match"`;

7. Add nvim integration tests (lua?).

8. Safely detach on panic leaving a log to be submitted as a GitHub issue.
*/

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
