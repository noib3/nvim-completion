use mlua::{
    prelude::{Lua, LuaResult},
    Table,
};
use neovim::{Api, Neovim};
use std::sync::{Arc, Mutex};

use crate::State;

pub fn setup(
    lua: &Lua,
    api: &Api,
    state: &Arc<Mutex<State>>,
) -> LuaResult<()> {
    let _state = state.clone();
    let start = lua.create_function(move |lua, opts: Table| {
        let bang = opts.get::<_, bool>("bang")?;
        super::compleet_start(lua, &mut _state.lock().unwrap(), bang)
    })?;

    let _state = state.clone();
    let stop = lua.create_function(move |lua, opts: Table| {
        let bang = opts.get::<_, bool>("bang")?;
        let api = Neovim::new(lua)?.api;
        super::compleet_stop(&api, &mut _state.lock().unwrap(), bang)
    })?;

    let opts = lua.create_table_from([("bang", true)])?;

    api.add_user_command("CompleetStart", start, opts.clone())?;
    api.add_user_command("CompleetStop", stop, opts)?;

    Ok(())
}
