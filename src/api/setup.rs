use mlua::{
    prelude::{LuaError, LuaResult, LuaValue},
    Lua, LuaSerdeExt,
};
use neovim::Neovim;
use std::sync::{Arc, Mutex};

use crate::settings::Settings;
use crate::state::State;
use crate::{autocmds, commands, hlgroups, mappings};

/// Executed by the `require("compleet").setup` Lua function.
pub fn setup(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    preferences: LuaValue,
) -> LuaResult<()> {
    let api = Neovim::new(lua)?.api;

    // If the Neovim version isn't >= 0.7 we echo an error message and return
    // early.
    if !api.call_function::<_, u8>("has", vec!["nvim-0.7"])? == 1 {
        let chunks = [
            ("[nvim-compleet]", Some("ErrorMsg")),
            (" Neovim v0.7+ is required.", None),
        ];
        api.echo(&chunks, true)?;
        return Ok(());
    }

    let _state = state.clone();
    let _state = &mut _state.lock().unwrap();

    _state.settings = match preferences {
        LuaValue::Table(t) => {
            match lua.from_value::<Settings>(LuaValue::Table(t)) {
                Ok(settings) => settings,

                Err(e) => match e {
                    LuaError::DeserializeError(msg) => {
                        let chunks = [
                            ("[nvim-compleet]", Some("ErrorMsg")),
                            (&format!(" {}", msg), None),
                        ];
                        api.echo(&chunks, true)?;
                        return Ok(());
                    },

                    _ => return Err(e),
                },
            }
        },

        LuaValue::Nil => Settings::default(),

        _ => {
            let chunks = [
                ("[nvim-compleet]", Some("ErrorMsg")),
                (" Invalid value '", None),
                (&format!("{:?}", preferences), Some("Statement")),
                ("'. Please pass either a table or ", None),
                ("nil", Some("Statement")),
                (" to the setup function", None),
            ];
            api.echo(&chunks, true)?;
            return Ok(());
        },
    };

    // // Used for debugging.
    // let nvim = Neovim::new(lua)?;
    // nvim.print(format!("{:?}", &_state.settings))?;

    // Only execute this block the first time this function is called.
    if !_state.did_setup {
        // Save the id of the `Compleet` augroup.
        _state.augroup_id = Some(autocmds::setup(lua, &api, state)?);

        commands::setup(lua, &api, state)?;
        hlgroups::setup(lua, &api)?;
        mappings::setup(lua, &api, state)?;

        _state.did_setup = true;
    }

    // // See how many times the state has been cloned across all the various
    // // functions.
    // let nvim = Neovim::new(lua)?;
    // nvim.print(format!(
    //     "State cloned {} times in total!",
    //     Arc::<Mutex<State>>::strong_count(state)
    // ))?;

    Ok(())
}
