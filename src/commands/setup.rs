use mlua::prelude::{Lua, LuaResult};
use neovim::{Api, Neovim};
use std::sync::{Arc, Mutex};

use crate::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<()> {
    let _state = state.clone();
    let stop = lua.create_function(move |lua, ()| {
        let api = Neovim::new(lua)?.api;
        super::compleet_stop(&api, &mut _state.lock().unwrap())
    })?;

    let _state = state.clone();
    let start = lua
        .create_function(move |lua, ()| super::compleet_start(lua, &_state))?;

    let empty = lua.create_table()?;
    api.add_user_command("CompleetStart", start, empty.clone())?;
    api.add_user_command("CompleetStop", stop, empty.clone())?;

    Ok(())
}
