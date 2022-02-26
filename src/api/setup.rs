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

    // ------------ INSERT COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let insert_completion = lua.create_function(move |lua, ()| {
        super::insert_completion(
            lua,
            &mut completion_state.lock().unwrap(),
            &mut ui_state.lock().unwrap(),
        )?;
        Ok(())
    })?;

    set_keymap.call::<_, ()>((
        "i",
        "<Plug>(compleet-insert-completion)",
        insert_completion,
        opts.clone(),
    ))?;

    // ------------ SELECT COMPLETION -----------
    let completion_state = Arc::clone(&state.completion);
    let ui_state = Arc::clone(&state.ui);
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(
            lua,
            &mut ui_state.lock().unwrap(),
            &completion_state.lock().unwrap(),
            step,
        )?;
        Ok(())
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
