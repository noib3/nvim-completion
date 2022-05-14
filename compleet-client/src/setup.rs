use std::{cell::RefCell, rc::Rc};

use mlua::{serde::Deserializer, Lua, Table};
use serde_path_to_error::deserialize;

use crate::autocmds;
use crate::client::Client;
use crate::commands;
use crate::mappings;
use crate::messages;
use crate::settings::Settings;

pub fn setup(
    lua: &Lua,
    state: &Rc<RefCell<Client>>,
    preferences: Option<Table>,
) -> mlua::Result<()> {
    let client = &mut state.borrow_mut();

    if client.did_setup() {
        messages::echoerr!(lua, "Can't setup more than once per session")?;
        return Ok(());
    }

    // Setup the highlight groups used to display warning/error messages.
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

    commands::setup(lua, state)?;
    mappings::setup(lua, state)?;

    let autocmds = autocmds::setup(state);

    client.setup(lua, autocmds, settings)?;

    Ok(())
}
