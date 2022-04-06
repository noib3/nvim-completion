use std::{cell::RefCell, rc::Rc};

use mlua::{
    prelude::{Lua, LuaError, LuaResult, LuaValue},
    serde::Deserializer,
};
use serde_path_to_error::deserialize;

use crate::{
    autocmds::Augroup,
    bindings::{nvim, r#fn},
    channel::Channel,
    commands,
    hlgroups,
    mappings,
    settings::Settings,
    state::State,
    ui::Ui,
    utils,
};

/// Executed by the `require("compleet").setup` Lua function.
pub fn setup(
    lua: &Lua,
    state: &Rc<RefCell<State>>,
    preferences: LuaValue,
) -> LuaResult<()> {
    // Setup the highlight groups used in the error messages.
    hlgroups::setup_error_msg(lua)?;

    // If the Neovim version isn't 0.7+ we echo an error message and return
    // early.
    if !r#fn::has(lua, "nvim-0.7")? {
        utils::echoerr(lua, "Neovim v0.7+ is required")?;
        return Ok(());
    }

    // Try to merge the `preferences` table with the default settings, echoing
    // an error message and returning early if something goes wrong.
    let settings = match preferences {
        LuaValue::Nil => Settings::default(),

        LuaValue::Table(t) => {
            let deserializer = Deserializer::new(LuaValue::Table(t));
            match deserialize::<_, Settings>(deserializer) {
                Ok(settings) => settings,

                Err(e) => match e.inner() {
                    // If the deserialization failed because of a
                    // badly-configured option we print an informative error
                    // message and return.
                    LuaError::DeserializeError(msg) => {
                        utils::echoerr(
                            lua,
                            format!(
                                "Error for `{}`: {}",
                                e.path(),
                                msg.replace("`", "\"")
                            ),
                        )?;
                        return Ok(());
                    },

                    // All other errors are propagated up.
                    _ => return Err(e.into_inner()),
                },
            }
        },

        _ => {
            utils::echoerr(
                lua,
                format!(
                    "Invalid value \"{}\". The setup function accepts either \
                     a table or `nil`",
                    nvim::inspect(lua, preferences)?
                ),
            )?;
            return Ok(());
        },
    };

    crate::bindings::nvim::print(lua, format!("{:#?}", settings))?;

    // If there aren't any sources enabled we echo a warning message and
    // return.
    if settings.sources.is_empty() {
        utils::echowar(
            lua,
            "All sources are disabled, I'm more useless than nipples on a man",
        )?;
        return Ok(());
    }

    // Update the state if this is the first time this function is called.
    let borrowed = &mut state.borrow_mut();
    if !borrowed.did_setup {
        borrowed.did_setup = true;

        hlgroups::setup(lua)?;
        commands::setup(lua, state)?;
        mappings::setup(lua, state)?;

        borrowed.augroup = Augroup::new(lua, state)?;
        borrowed.augroup.set(lua)?;

        borrowed.channel = Channel::new(lua, state)?;
        borrowed.ui = Ui::new(lua, &settings.ui)?;
        borrowed.settings = settings;
    }

    Ok(())
}
