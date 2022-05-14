use std::{cell::RefCell, rc::Rc};

use bindings::opinionated::buffer::{LuaFn, OnBytesSignature};
use mlua::Lua;

use crate::client::Client;

pub fn setup(
    client: &Rc<RefCell<Client>>,
) -> (
    LuaFn<(), ()>,
    LuaFn<(), ()>,
    LuaFn<(), ()>,
    LuaFn<OnBytesSignature, Option<bool>>,
) {
    // TODO: do I need to specify `: &Lua`?

    let cloned = client.clone();
    let on_insert_leave = Box::new(move |lua: &Lua, ()| {
        super::on_insert_leave(lua, &mut cloned.borrow_mut())
    });

    let cloned = client.clone();
    let on_cursor_moved_i = Box::new(move |lua: &Lua, ()| {
        super::on_cursor_moved_i(lua, &mut cloned.borrow_mut())
    });

    let cloned = client.clone();
    let on_buf_enter = Box::new(move |lua: &Lua, ()| {
        super::on_buf_enter(lua, &mut cloned.borrow_mut())
    });

    let cloned = client.clone();
    let on_bytes = Box::new(move |lua: &Lua, args: OnBytesSignature| {
        // let client = &mut cloned.borrow_mut();
        // let edit = client.apply_on_bytes(args);
        // super::on_bytes(lua, client, edit)
        super::on_bytes(lua, &mut cloned.borrow_mut(), args)
    });

    (on_insert_leave, on_cursor_moved_i, on_buf_enter, on_bytes)
}
