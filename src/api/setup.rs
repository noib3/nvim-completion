use mlua::{Function, Lua, Result, Table};
use std::sync::Arc;

use crate::{Nvim, State};

pub fn setup(lua: &Lua, state: &State) -> Result<()> {
    let nvim = Nvim::new(lua)?;
    setup_augroups(&nvim)?;
    setup_plug_mappings(lua, state)?;

    Ok(())
}

fn setup_augroups(nvim: &Nvim) -> Result<()> {
    nvim.exec(
        r#"
    augroup compleet_events
      autocmd CursorMovedI * lua require("compleet").__events.cursor_moved()
      autocmd InsertLeave  * lua require("compleet").__events.insert_left()
      autocmd TextChangedI * lua require("compleet").__events.text_changed()
    augroup END
"#,
        false,
    )
}

fn setup_plug_mappings(lua: &Lua, state: &State) -> Result<()> {
    let set_keymap = lua
        .globals()
        .get::<&str, Table>("vim")?
        .get::<&str, Table>("keymap")?
        .get::<&str, Function>("set")?;

    let opts = lua.create_table()?;
    opts.set("silent", true)?;

    // ------------ ACCEPT COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let accept_completion = lua.create_function(move |lua, ()| {
        super::accept_completion(
            lua,
            &mut completion_state.lock().unwrap(),
            &mut ui_state.lock().unwrap(),
        )?;
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-accept-completion)",
        accept_completion,
        opts.clone(),
    ))?;

    // ------------ SELECT NEXT COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let select_next_completion = lua.create_function(move |lua, ()| {
        super::select_next_completion(
            lua,
            &mut ui_state.lock().unwrap(),
            completion_state.lock().unwrap().completion_items.len(),
        )?;
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-next-completion)",
        select_next_completion,
        opts.clone(),
    ))?;

    // ------------ SELECT PREV COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let select_prev_completion = lua.create_function(move |lua, ()| {
        super::select_prev_completion(
            lua,
            &mut ui_state.lock().unwrap(),
            completion_state.lock().unwrap().completion_items.len(),
        )?;
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-prev-completion)",
        select_prev_completion,
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
        )?;
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-show-completions)",
        show_completions,
        opts.clone(),
    ))?;

    Ok(())
}
