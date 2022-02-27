use mlua::{Function, Lua, Result, Table};
use std::sync::Arc;

use crate::config::{self, Config};
use crate::{Nvim, State};

pub fn setup(
    lua: &Lua,
    state: &State,
    preferences: Option<Table>,
) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    let config = Arc::clone(&state.config);
    let mut config = config.lock().unwrap();
    *config = match Config::try_from(preferences) {
        Ok(config) => config,
        Err(e) => match e {
            config::Error::Conversion { option, expected } => {
                nvim.echo(
                    &[
                        &["[nvim-compleet]: ", "ErrorMsg"],
                        &["Error parsing config option '"],
                        &[option, "TSWarning"],
                        &[&format!("': expected a {expected}.")],
                    ],
                    true,
                    &[],
                )?;

                return Ok(());
            },

            config::Error::OptionDoesntExist { option } => {
                nvim.echo(
                    &[
                        &["[nvim-compleet]: ", "ErrorMsg"],
                        &["Config option '"],
                        &[&option, "TSWarning"],
                        &["' doesn't exist!"],
                    ],
                    true,
                    &[],
                )?;

                return Ok(());
            },

            config::Error::Lua(e) => return Err(e),
        },
    };

    // let print = lua.globals().get::<&str, Function>("print")?;
    // print.call::<_, ()>(format!("{:?}", &config))?;

    setup_augroups(&nvim)?;
    setup_plug_mappings(lua, state)?;
    if config.enable_default_mappings {
        add_default_mappings(lua, state)?;
    }

    Ok(())
}

// Use https://github.com/neovim/neovim/pull/14661 once it gets merged into a
// stable release.
fn setup_augroups(nvim: &Nvim) -> Result<()> {
    let src = r#"
    augroup compleet_events
      autocmd CursorMovedI * lua require("compleet").__events.cursor_moved()
      autocmd InsertLeave  * lua require("compleet").__events.insert_left()
      autocmd TextChangedI * lua require("compleet").__events.text_changed()
    augroup END
"#;
    nvim.exec(src, false)
}

fn setup_plug_mappings(lua: &Lua, state: &State) -> Result<()> {
    let set_keymap = lua
        .globals()
        .get::<&str, Table>("vim")?
        .get::<&str, Table>("keymap")?
        .get::<&str, Function>("set")?;

    let opts = lua.create_table()?;
    opts.set("silent", true)?;

    // TODO: unify w/ insert selected completion
    // ------------ INSERT HINTED COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let insert_hinted_completion = lua.create_function(move |lua, ()| {
        if let Some(index) =
            &ui_state.lock().unwrap().completion_hint.hinted_index
        {
            super::insert_completion(
                lua,
                &mut completion_state.lock().unwrap(),
                *index,
            )?;
        }
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-insert-hinted-completion)",
        insert_hinted_completion,
        opts.clone(),
    ))?;

    // ------------ INSERT SELECTED COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let insert_selected_completion = lua.create_function(move |lua, ()| {
        if let Some(index) =
            &ui_state.lock().unwrap().completion_menu.selected_index
        {
            super::insert_completion(
                lua,
                &mut completion_state.lock().unwrap(),
                *index,
            )?;
        }
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-insert-selected-completion)",
        insert_selected_completion,
        opts.clone(),
    ))?;

    // ------------ SELECT COMPLETION -----------
    let config = Arc::clone(&state.config);
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(
            lua,
            &config.lock().unwrap(),
            &mut ui_state.lock().unwrap(),
            &completion_state.lock().unwrap(),
            step,
        )
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-next-completion)",
        select_completion.bind(1)?,
        opts.clone(),
    ))?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-prev-completion)",
        select_completion.bind(-1)?,
        opts.clone(),
    ))?;

    // ------------ SHOW COMPLETIONS -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let show_completions = lua.create_function(move |lua, ()| {
        super::show_completions(
            lua,
            &completion_state.lock().unwrap(),
            &mut ui_state.lock().unwrap(),
        )
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-show-completions)",
        show_completions,
        opts.clone(),
    ))?;

    Ok(())
}

fn add_default_mappings(lua: &Lua, state: &State) -> Result<()> {
    let set_keymap = lua
        .globals()
        .get::<&str, Table>("vim")?
        .get::<&str, Table>("keymap")?
        .get::<&str, Function>("set")?;

    let opts = lua.create_table()?;
    opts.set("expr", true)?;
    opts.set("remap", true)?;

    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let tab = lua.create_function(move |lua, ()| -> Result<&'static str> {
        Ok(
            if super::is_completion_menu_visible(&ui_state.lock().unwrap()) {
                "<Plug>(compleet-next-completion)"
            } else if super::has_completions(
                lua,
                &mut completion_state.lock().unwrap(),
            )? {
                "<Plug>(compleet-show-completions)"
            } else {
                "<Tab>"
            },
        )
    })?;

    let ui_state = Arc::clone(&state.ui);
    let s_tab = lua.create_function(move |_, ()| -> Result<&'static str> {
        Ok(
            if super::is_completion_menu_visible(&ui_state.lock().unwrap()) {
                "<Plug>(compleet-prev-completion)"
            } else {
                "<S-Tab>"
            },
        )
    })?;

    let ui_state = Arc::clone(&state.ui);
    let cr = lua.create_function(move |_, ()| -> Result<&'static str> {
        Ok(
            if super::is_completion_item_selected(&ui_state.lock().unwrap()) {
                "<Plug>(compleet-insert-selected-completion)"
            } else {
                "<CR>"
            },
        )
    })?;

    set_keymap.call::<_, ()>(("i", "<Tab>", tab, opts.clone()))?;
    set_keymap.call::<_, ()>(("i", "<S-Tab>", s_tab, opts.clone()))?;
    set_keymap.call::<_, ()>(("i", "<CR>", cr, opts.clone()))?;

    Ok(())
}
