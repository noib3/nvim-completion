use std::{cell::RefCell, rc::Rc};

use mlua::{
    prelude::{Lua, LuaResult},
    Table,
};

use crate::bindings::api;
use crate::State;

pub fn setup(lua: &Lua, state: &Rc<RefCell<State>>) -> LuaResult<()> {
    let cloned = state.clone();
    let start = lua.create_function(move |lua, opts: Table| {
        let bang = opts.get::<_, bool>("bang")?;
        super::compleet_start(lua, &mut cloned.borrow_mut(), bang)
    })?;

    let _state = state.clone();
    let stop = lua.create_function(move |lua, opts: Table| {
        let bang = opts.get::<_, bool>("bang")?;
        super::compleet_stop(lua, &mut _state.borrow_mut(), bang)
    })?;

    let opts = lua.create_table_from([("bang", true)])?;

    api::add_user_command(lua, "CompleetStart", start, opts.clone())?;
    api::add_user_command(lua, "CompleetStop", stop, opts)?;

    Ok(())
}
