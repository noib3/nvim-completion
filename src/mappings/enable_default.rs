use mlua::prelude::{Lua, LuaResult};
use neovim::{Api, Neovim};
use std::sync::{Arc, Mutex};

use crate::api;
use crate::state::State;

/// Creates some default insert mode mappings for `<Tab>`, `<S-Tab>` and
/// `<CR>`.
pub fn enable_default(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<()> {
    // Insert mode mapping for `<Tab>`. If the completion menu is visible
    // select the next completion, if it isn't but there are completions to be
    // displayed show the completion menu, else just return `<Tab>`.
    let _state = state.clone();
    let tab = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        let api = Neovim::new(lua)?.api;

        let str = if _state.ui.completion_menu.is_visible() {
            "<Plug>(compleet-next-completion)"
        } else if api::has_completions(lua, _state)? {
            "<Plug>(compleet-show-completions)"
        } else {
            "<Tab>"
        };

        api.replace_termcodes(str, true, true, true)
    })?;

    // Insert mode mapping for `<Tab>`. If the completion menu is visible
    // select the previous completion, else just return `<S-Tab>`.
    let _state = state.clone();
    let s_tab = lua.create_function(move |lua, ()| {
        let api = Neovim::new(lua)?.api;

        let str = if _state.lock().unwrap().ui.completion_menu.is_visible() {
            "<Plug>(compleet-prev-completion)"
        } else {
            "<S-Tab>"
        };

        api.replace_termcodes(str, true, true, true)
    })?;

    // Insert mode mapping for `<Tab>`. If a completion item in the completion
    // menu is currently selected insert it, else just return `<CR>`.
    let _state = state.clone();
    let cr = lua.create_function(move |lua, ()| {
        let api = Neovim::new(lua)?.api;

        let str =
            if _state.lock().unwrap().ui.completion_menu.is_item_selected() {
                "<Plug>(compleet-insert-selected-completion)"
            } else {
                "<CR>"
            };

        api.replace_termcodes(str, true, true, true)
    })?;

    let opts = lua.create_table_from([("expr", true), ("noremap", false)])?;

    opts.set("callback", tab)?;
    api.set_keymap("i", "<Tab>", "", opts.clone())?;

    opts.set("callback", s_tab)?;
    api.set_keymap("i", "<S-Tab>", "", opts.clone())?;

    opts.set("callback", cr)?;
    api.set_keymap("i", "<CR>", "", opts.clone())?;

    Ok(())
}
