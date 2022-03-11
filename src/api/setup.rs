use mlua::{Function, Lua, Result, Table};
use neovim::{Api, Keymap, Neovim};
use std::sync::{Arc, Mutex};

use crate::settings::{Error, Settings};
use crate::State;

const COMPLEET_AUGROUP_NAME: &'static str = "Compleet";

/// Executed on every call to `require("compleet").setup({..})`.
pub fn setup(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    preferences: Option<Table>,
) -> Result<()> {
    let _state = state.clone();
    let _state = &mut _state.lock().unwrap();

    let nvim = Neovim::new(lua)?;
    let api = &nvim.api;

    _state.settings = match Settings::try_from(preferences) {
        Ok(settings) => settings,
        Err(e) => match e {
            Error::OptionDoesntExist { option } => {
                api.echo(
                    &[
                        ("[nvim-compleet]: ", "ErrorMsg"),
                        ("Config option '", ""),
                        (&option, "Statement"),
                        ("", "' doesn't exist!"),
                    ],
                    true,
                )?;

                return Ok(());
            },

            Error::FailedConversion { option, expected } => {
                api.echo(
                    &[
                        ("[nvim-compleet]: ", "ErrorMsg"),
                        ("Error parsing config option '", ""),
                        (option, "Statement"),
                        (&format!("': expected a {expected}."), ""),
                    ],
                    true,
                )?;

                return Ok(());
            },

            Error::InvalidValue { option, reason } => {
                api.echo(
                    &[
                        ("[nvim-compleet]: ", "ErrorMsg"),
                        ("Invalid value for config option '", ""),
                        (&option, "Statement"),
                        (&format!("': {reason}."), ""),
                    ],
                    true,
                )?;

                return Ok(());
            },

            Error::Lua(e) => return Err(e),
        },
    };

    // nvim.print(format!("{:?}", &config))?;

    _state.ui.completion_menu.max_height = _state.settings.max_menu_height;

    setup_augroups(lua, api, state)?;
    setup_hlgroups(lua, api)?;
    setup_mappings(lua, &nvim.keymap, state)?;

    if _state.settings.enable_default_mappings {
        enable_default_mappings(lua, &nvim.keymap, state)?;
    }

    Ok(())
}

fn setup_augroups(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> Result<()> {
    let _state = state.clone();
    let cleanup = lua.create_function(move |lua, ()| {
        super::cleanup_ui(lua, &mut _state.lock().unwrap().ui)
    })?;

    let _state = state.clone();
    let maybe_show_hint = lua.create_function(move |lua, ()| {
        super::maybe_show_hint(lua, &mut _state.lock().unwrap())
    })?;

    let _state = state.clone();
    let text_changed = lua.create_function(move |lua, ()| {
        super::text_changed(lua, &mut _state.lock().unwrap())
    })?;

    let opts = lua.create_table_with_capacity(0, 1)?;
    opts.set("clear", true)?;
    let _group_id = api.create_augroup(COMPLEET_AUGROUP_NAME, opts)?;

    let opts_w_callback = |callback: Function| -> Result<Table> {
        let opts = lua.create_table_with_capacity(0, 2)?;
        // TODO: why doesn't it work if I use the group id returned by
        // `create_augroup` here instead of the name?
        // opts.set("group", group_id)?;
        opts.set("group", COMPLEET_AUGROUP_NAME)?;
        opts.set("callback", callback)?;
        Ok(opts)
    };

    api.create_autocmd(
        &["CursorMovedI", "InsertLeave"],
        opts_w_callback(cleanup)?,
    )?;
    api.create_autocmd(
        &["CursorMovedI", "InsertEnter"],
        opts_w_callback(maybe_show_hint)?,
    )?;
    api.create_autocmd(&["TextChangedI"], opts_w_callback(text_changed)?)?;

    Ok(())
}

fn setup_hlgroups(lua: &Lua, api: &Api) -> Result<()> {
    // TODO: make something like this work
    // nvim.set_hl(0, "CompleetMenu", lua.t! { link = "NormalFloat" })?;

    // `CompleetMenu`
    // Used to highlight the completion menu.
    let opts = lua.create_table_from([("link", "NormalFloat")])?;
    api.set_hl(0, "CompleetMenu", opts)?;

    // `CompleetHint`
    // Used to highlight the completion hint.
    let opts = lua.create_table_from([("link", "Comment")])?;
    api.set_hl(0, "CompleetHint", opts)?;

    // `CompleetDetails`
    // Used to highlight the details window.
    let opts = lua.create_table_from([("link", "NormalFloat")])?;
    api.set_hl(0, "CompleetDetails", opts)?;

    // `CompleetMenuSelected`
    // Used to highlight the currently selected completion item.
    let opts = lua.create_table_from([("link", "PmenuSel")])?;
    api.set_hl(0, "CompleetMenuSelected", opts)?;

    // `CompleetMenuMatchingChars`
    // Used to highlight the characters where a completion item matches the
    // current prefix.
    let opts = lua.create_table_from([("link", "Statement")])?;
    api.set_hl(0, "CompleetMenuMatchingChars", opts)?;

    Ok(())
}

fn setup_mappings(
    lua: &Lua,
    keymap: &Keymap,
    state: &Arc<Mutex<State>>,
) -> Result<()> {
    // Insert the currently hinted completion
    let _state = state.clone();
    let insert_hinted_completion = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        if let Some(index) = _state.ui.completion_hint.hinted_index {
            super::insert_completion(lua, &mut _state.completion, index)?;
        }
        Ok(())
    })?;

    // Insert the currently selected completion
    let _state = state.clone();
    let insert_selected_completion = lua.create_function(move |lua, ()| {
        let _state = &mut _state.lock().unwrap();
        if let Some(index) = _state.ui.completion_menu.selected_completion {
            super::insert_completion(lua, &mut _state.completion, index)?;
        }
        Ok(())
    })?;

    // Select either the previous or next completion in the completion menu
    // based on the value of `step`.
    let _state = state.clone();
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(lua, &mut _state.lock().unwrap(), step)
    })?;

    // Show the completion menu with all the currently available completion
    // candidates.
    let _state = state.clone();
    let show_completions = lua.create_function(move |lua, ()| {
        super::show_completions(lua, &mut _state.lock().unwrap())
    })?;

    let opts = lua.create_table_with_capacity(0, 1)?;
    opts.set("silent", true)?;

    keymap.set(
        "i",
        "<Plug>(compleet-insert-hinted-completion)",
        insert_hinted_completion,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-insert-selected-completion)",
        insert_selected_completion,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-next-completion)",
        select_completion.bind(1)?,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-prev-completion)",
        select_completion.bind(-1)?,
        Some(opts.clone()),
    )?;

    keymap.set(
        "i",
        "<Plug>(compleet-show-completions)",
        show_completions,
        Some(opts.clone()),
    )?;

    Ok(())
}

fn enable_default_mappings(
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
        } else if super::has_completions(lua, &mut _state.completion)? {
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

    let opts = lua.create_table_with_capacity(0, 2)?;
    opts.set("expr", true)?;
    opts.set("remap", true)?;

    keymap.set("i", "<Tab>", tab, Some(opts.clone()))?;
    keymap.set("i", "<S-Tab>", s_tab, Some(opts.clone()))?;
    keymap.set("i", "<CR>", cr, Some(opts.clone()))?;

    Ok(())
}
