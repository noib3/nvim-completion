use std::{cell::RefCell, rc::Rc};

use bindings::api;
use mlua::{Lua, Table};

use super::{compleet_start, compleet_stop};
use crate::Client;

pub fn setup(lua: &Lua, client: &Rc<RefCell<Client>>) -> mlua::Result<()> {
    let cloned = client.clone();
    let start = lua.create_function(move |lua, opts: Table| {
        let client = &mut cloned.borrow_mut();
        if opts.get::<_, bool>("bang")? {
            compleet_start::attach_all(lua, client)
        } else {
            compleet_start::attach_current(lua, client)
        }
    })?;

    let cloned = client.clone();
    let stop = lua.create_function(move |lua, opts: Table| {
        let state = &mut cloned.borrow_mut();
        if opts.get::<_, bool>("bang")? {
            compleet_stop::detach_all(lua, state)
        } else {
            compleet_stop::detach_current(lua, state)
        }
    })?;

    let opts = lua.create_table_from([("bang", true)])?;

    api::create_user_command(lua, "CompleetStart", start, opts.clone())?;
    api::create_user_command(lua, "CompleetStop", stop, opts)?;

    Ok(())
}

// pub fn setup(client: &Rc<RefCell<Client>>) -> nvim::Result<()> {
//     let cloned = client.clone();
//     let start = move |opts| {
//         let client = &mut cloned.borrow_mut();
//         match opts.bang {
//             true => compleet_stop::attach_all(client),
//             _ => compleet_stop::attach_current(client),
//         }
//     };

//     let cloned = client.clone();
//     let stop = move |opts| {
//         let client = &mut cloned.borrow_mut();
//         match opts.bang {
//             true => compleet_stop::detach_all(client),
//             _ => compleet_stop::detach_current(client),
//         }
//     };

//     let opts = UserCommandOptsBuilder::new().bang().build();

//     api::create_user_command("CompleetStart", start, &opts)?;
//     api::create_user_command("CompleetStop", stop, &opts)?;

//     Ok(())
// }
