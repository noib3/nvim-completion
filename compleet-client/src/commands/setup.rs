use std::{cell::RefCell, rc::Rc};

use mlua::{
    prelude::{Lua, LuaResult},
    Table,
};

use super::{compleet_start, compleet_stop};
use crate::bindings::api;
use crate::State;

pub fn setup(lua: &Lua, state: &Rc<RefCell<State>>) -> LuaResult<()> {
    let cloned = state.clone();
    let start = lua.create_function(move |lua, opts: Table| {
        let state = &mut cloned.borrow_mut();
        if opts.get::<_, bool>("bang")? {
            // `CompleetStart!` attaches to all the buffers.
            compleet_start::attach_all(lua, state)
        } else {
            // `CompleetStart` only attaches the current buffer.
            compleet_start::attach_current(lua, state)
        }
    })?;

    let cloned = state.clone();
    let stop = lua.create_function(move |lua, opts: Table| {
        let state = &mut cloned.borrow_mut();
        if opts.get::<_, bool>("bang")? {
            // `CompleetStop!` detached from all the buffers.
            compleet_stop::detach_all(lua, state)
        } else {
            // `CompleetStop` only detaches the current buffer.
            compleet_stop::detach_current(lua, state)
        }
    })?;

    let opts = lua.create_table_from([("bang", true)])?;

    api::create_user_command(lua, "CompleetStart", start, opts.clone())?;
    api::create_user_command(lua, "CompleetStop", stop, opts)?;

    Ok(())
}
