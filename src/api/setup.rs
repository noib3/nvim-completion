use mlua::{Lua, Result, Table};
use neovim::Neovim;
use std::sync::{Arc, Mutex};

use crate::settings::{Error, Settings};
use crate::state::State;
use crate::{autocmds, commands, hlgroups, mappings};

/// Executed by the `require("compleet").setup` Lua function.
pub fn setup(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    preferences: Option<Table>,
) -> Result<()> {
    let nvim = Neovim::new(lua)?;
    let api = &nvim.api;

    // If the Neovim version isn't >= 0.7 we echo an error message and return
    // early.
    if !api.call_function::<_, u8>("has", vec!["nvim-0.7"])? == 1 {
        nvim.api.echo(
            &[
                ("[nvim-compleet]: ", Some("ErrorMsg")),
                ("Neovim v0.7+ is required.", None),
            ],
            true,
        )?;

        return Ok(());
    }

    let _state = state.clone();
    let _state = &mut _state.lock().unwrap();

    _state.settings = match Settings::try_from(preferences) {
        Ok(settings) => settings,

        Err(e) => {
            let mut chunks = match e {
                Error::OptionDoesntExist { option } => vec![
                    ("Config option '".into(), None),
                    (option, Some("Statement")),
                    ("' doesn't exist!".into(), None),
                ],

                Error::FailedConversion { option, expected } => vec![
                    ("Config option '".into(), None),
                    ("Error parsing config option '".into(), None),
                    (option.into(), Some("Statement")),
                    (format!("': expected a {expected}."), None),
                ],

                Error::InvalidValue { option, reason } => vec![
                    ("Invalid value for config option '".into(), None),
                    (option.into(), Some("Statement")),
                    (format!("': {reason}."), None),
                ],

                Error::Lua(e) => return Err(e),
            };

            chunks.insert(0, ("[nvim-compleet]: ".into(), Some("ErrorMsg")));
            api.echo(&chunks, true)?;

            return Ok(());
        },
    };

    // Used for debugging.
    // nvim.print(format!("{:?}", &_state.settings))?;

    // Save the `id` of the `Compleet` augroup.
    _state.augroup_id = Some(autocmds::setup(lua, api, state)?);

    commands::setup(lua, api, state)?;
    hlgroups::setup(lua, api)?;
    mappings::setup(lua, &nvim.keymap, state)?;

    if _state.settings.enable_default_mappings {
        mappings::enable_default(lua, &nvim.keymap, state)?;
    }

    // // See how many times the state has been cloned across all the various
    // // functions.
    // nvim.print(format!(
    //     "State cloned {} times in total!",
    //     Arc::<Mutex<State>>::strong_count(state)
    // ))?;

    Ok(())
}
