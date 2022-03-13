use mlua::{Lua, Result, Table};
use neovim::Neovim;
use std::sync::{Arc, Mutex};

use crate::settings::{Error, Settings};
use crate::state::State;
use crate::{autocmds, hlgroups, mappings};

/// Executed on every call to `require("compleet").setup({..})`.
pub fn setup(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    preferences: Option<Table>,
) -> Result<()> {
    let nvim = Neovim::new(lua)?;

    if !nvim.api.call_function::<_, usize>("has", &["nvim-0.7"])? == 1 {
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
        Err(e) => match e {
            Error::OptionDoesntExist { option } => {
                nvim.api.echo(
                    &[
                        ("[nvim-compleet]: ", Some("ErrorMsg")),
                        ("Config option '", None),
                        (&option, Some("Statement")),
                        ("' doesn't exist!", None),
                    ],
                    true,
                )?;

                return Ok(());
            },

            Error::FailedConversion { option, expected } => {
                nvim.api.echo(
                    &[
                        ("[nvim-compleet]: ", Some("ErrorMsg")),
                        ("Error parsing config option '", None),
                        (option, Some("Statement")),
                        (&format!("': expected a {expected}."), None),
                    ],
                    true,
                )?;

                return Ok(());
            },

            Error::InvalidValue { option, reason } => {
                nvim.api.echo(
                    &[
                        ("[nvim-compleet]: ", Some("ErrorMsg")),
                        ("Invalid value for config option '", None),
                        (&option, Some("Statement")),
                        (&format!("': {reason}."), None),
                    ],
                    true,
                )?;

                return Ok(());
            },

            Error::Lua(e) => return Err(e),
        },
    };

    // nvim.print(format!("{:?}", &_state.settings))?;

    _state.ui.completion_menu.max_height = _state.settings.max_menu_height;

    autocmds::setup(lua, &nvim.api, state)?;
    hlgroups::setup(lua, &nvim.api)?;
    mappings::setup(lua, &nvim.keymap, state)?;

    if _state.settings.enable_default_mappings {
        mappings::enable_default(lua, &nvim.keymap, state)?;
    }

    Ok(())
}
