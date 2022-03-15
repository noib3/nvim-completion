use mlua::{Lua, Result};
use neovim::Api;
use std::sync::{Arc, Mutex};

use crate::State;

pub fn setup(lua: &Lua, api: &Api, state: &Arc<Mutex<State>>) -> Result<()> {
    let _state = state.clone();
    let stop = lua.create_function(move |lua, ()| {
        super::compleet_stop(lua, &mut _state.lock().unwrap())
    })?;

    let _state = state.clone();
    let start = lua
        .create_function(move |lua, ()| super::compleet_start(lua, &_state))?;

    let empty = lua.create_table()?;
    api.add_user_command("CompleetStart", start, empty.clone())?;
    api.add_user_command("CompleetStop", stop, empty.clone())?;

    Ok(())
}
