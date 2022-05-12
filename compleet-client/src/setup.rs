use std::{cell::RefCell, rc::Rc};

use mlua::{prelude::Lua, serde::Deserializer, Table};
use serde_path_to_error::deserialize;

use crate::autocmds::Augroup;
use crate::channel::Channel;
use crate::client::Client;
use crate::commands;
use crate::hlgroups;
use crate::mappings;
use crate::messages;
use crate::settings::Settings;
use crate::ui::Ui;

/// Executed by the `require("compleet").setup` Lua function.
pub fn setup(
    lua: &Lua,
    state: &Rc<RefCell<Client>>,
    preferences: Option<Table>,
) -> mlua::Result<()> {
    // Setup the highlight groups used when displaying warning/error messages.
    messages::hlgroups::setup(lua)?;

    let settings = if let Some(table) = preferences {
        let deserializer = Deserializer::new(mlua::Value::Table(table));
        match deserialize::<_, Settings>(deserializer) {
            Ok(settings) => settings,

            Err(err) => match err.inner() {
                // If the deserialization failed because of a badly-configured
                // option we print an informative error message and return.
                mlua::Error::DeserializeError(msg) => {
                    let opt = err.path();
                    let msg = msg.replace("`", "\"");
                    messages::echoerr!(lua, "Error for `{opt}`: {msg}")?;
                    return Ok(());
                },

                // All other errors are bubbled up.
                _ => return Err(err.into_inner()),
            },
        }
    } else {
        Settings::default()
    };

    if settings.sources.is_empty() {
        messages::echowarn!(lua, "All sources are disabled")?;
        return Ok(());
    }

    // Update the state if this is the first time this function is called.
    // TODO: refactor
    let borrowed = &mut state.borrow_mut();
    if !borrowed.did_setup {
        commands::setup(lua, state)?;
        mappings::setup(lua, state)?;

        let Settings { ui, completion, sources } = settings;

        borrowed.channel = Some(Channel::new(lua, state, sources)?);
        borrowed.augroup = Augroup::new(lua, state)?;
        borrowed.augroup.set(lua)?;
        borrowed.did_setup = true;
        borrowed.ui = Ui::new(lua, &ui)?;
        borrowed.settings.ui = ui;
        borrowed.settings.completion = completion;
    }

    Ok(())
}
