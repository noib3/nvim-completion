use mlua::{Lua, Result};
use neovim::Keymap;
use std::sync::{Arc, Mutex};

use crate::api;
use crate::state::State;

pub fn enable_default(
    lua: &Lua,
    keymap: &Keymap,
    state: &Arc<Mutex<State>>,
) -> Result<()> {
    // Insert mode mapping for `<Tab>`. If the completion menu is visible
    // select the next completion, if it isn't but there are completions to be
    // displayed show the completion menu, else just return `<Tab>`.
    let _state = state.clone();
    let tab = lua.create_function(move |lua, ()| -> Result<&'static str> {
        let _state = &mut _state.lock().unwrap();
        if _state.ui.completion_menu.is_visible() {
            Ok("<Plug>(compleet-next-completion)")
        } else if api::has_completions(lua, _state)? {
            Ok("<Plug>(compleet-show-completions)")
        } else {
            Ok("<Tab>")
        }
    })?;

    // Insert mode mapping for `<Tab>`. If the completion menu is visible
    // select the previous completion, else just return `<S-Tab>`.
    let _state = state.clone();
    let s_tab = lua.create_function(move |_, ()| -> Result<&'static str> {
        if _state.lock().unwrap().ui.completion_menu.is_visible() {
            Ok("<Plug>(compleet-prev-completion)")
        } else {
            Ok("<S-Tab>")
        }
    })?;

    // Insert mode mapping for `<Tab>`. If a completion item in the completion
    // menu is currently selected insert it, else just return `<CR>`.
    let _state = state.clone();
    let cr = lua.create_function(move |_, ()| -> Result<&'static str> {
        if _state.lock().unwrap().ui.completion_menu.is_item_selected() {
            Ok("<Plug>(compleet-insert-selected-completion)")
        } else {
            Ok("<CR>")
        }
    })?;

    let opts = lua.create_table_from([("silent", true), ("remap", true)])?;

    keymap.set("i", "<Tab>", tab, Some(opts.clone()))?;
    keymap.set("i", "<S-Tab>", s_tab, Some(opts.clone()))?;
    keymap.set("i", "<CR>", cr, Some(opts.clone()))?;

    Ok(())
}
