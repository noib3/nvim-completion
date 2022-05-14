use std::{cell::RefCell, rc::Rc};

use bindings::api;
use mlua::Lua;

use crate::client::Client;

pub fn setup(lua: &Lua, client: &Rc<RefCell<Client>>) -> mlua::Result<()> {
    // Insert either the first or the selected completion into the buffer,
    // depending on the value of `first`.
    let cloned = client.clone();
    let insert_completion = lua.create_function(move |lua, first| {
        let mut client = cloned.borrow_mut();

        let maybe = if first {
            client.completions.get(0)
        } else {
            client.ui.menu.selected_index.map(|i| &client.completions[i])
        };

        if let Some(completion) = maybe {
            super::insert_completion(
                lua,
                &client.cursor,
                completion,
                client.matched_bytes,
            )?;
        }

        // If the `completion.after_inserting` option is set to `false` we
        // skip the next call to `on_bytes` so that completions are not
        // recomputed.
        if !client.settings.completion.after_inserting {
            client.ignore_next_on_bytes = true;
        }

        client.completions.clear();

        Ok(())
    })?;

    // Select either the previous or next completion in the completion menu
    // depending on the value of `step`.
    let cloned = client.clone();
    let select_completion = lua.create_function(move |lua, step| {
        super::select_completion(lua, &mut cloned.borrow_mut(), step)
    })?;

    // Show the completion menu with all the currently available completion
    // candidates.
    let cloned = client.clone();
    let show_completions = lua.create_function(move |lua, ()| {
        super::show_completions(lua, &mut cloned.borrow_mut())
    })?;

    let opts = lua.create_table_from([("silent", true)])?;

    opts.set("callback", insert_completion.bind(false)?)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-insert-selected-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", insert_completion.bind(true)?)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-insert-first-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", select_completion.bind(-1)?)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-prev-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", select_completion.bind(1)?)?;
    api::set_keymap(
        lua,
        "i",
        "<Plug>(compleet-next-completion)",
        "",
        opts.clone(),
    )?;

    opts.set("callback", show_completions)?;
    api::set_keymap(lua, "i", "<Plug>(compleet-show-completions)", "", opts)?;

    Ok(())
}
