use std::{cell::RefCell, rc::Rc};

use bindings::api;
use bindings::opinionated::buffer::{LuaFn, OnBytesSignature};
use mlua::prelude::{Lua, LuaFunction, LuaRegistryKey, LuaResult, LuaValue};

use crate::{channel, client::Client, constants::AUGROUP_NAME, ui};

pub type OnInsertLeave = LuaFn<(), ()>;
pub type CursorMovedI = LuaFn<(), ()>;

#[derive(Debug, Default)]
pub struct Augroup {
    /// The augroup id returned by `vim.api.nvim_create_autocmd`.
    id: Option<u16>,

    /// The registry key of the Lua function called on `BufEnter` to try to
    /// attach to the buffer.
    on_buf_enter_key: Option<LuaRegistryKey>,

    on_insert_leave: LuaFn<(), ()>,
    on_cursor_moved_i: LuaFn<(), ()>,
    on_buf_enter: LuaFn<(), ()>,
}

impl Augroup {
    pub fn new(lua: &Lua, client: &Rc<RefCell<Client>>) -> mlua::Result<Self> {
        let cloned = client.clone();
        let on_insert_leave = Rc::new(move |lua, ()| {
            super::on_insert_leave(lua, &mut cloned.borrow_mut())
        });

        let cloned = client.clone();
        let on_cursor_moved_i = Rc::new(move |lua, ()| {
            super::on_cursor_moved_i(lua, &mut cloned.borrow_mut())
        });

        let cloned = client.clone();
        let on_bytes = Rc::new(move |lua, args: OnBytesSignature| {
            // let client = &mut cloned.borrow_mut();
            // let edit = client.apply_on_bytes(args);
            // channel::on_bytes(lua, client, edit)
            channel::on_bytes(lua, &mut cloned.borrow_mut(), args)
        });

        let cloned = client.clone();
        let on_buf_enter = Rc::new(move |lua, ()| {
            super::on_buf_enter(
                lua,
                &mut clone.borrow_mut(),
                on_insert_leave.clone(),
                on_cursor_moved_i.clone(),
                on_bytes.clone(),
            )
        });

        Ok(Self {
            on_buf_enter_key: Some(lua.create_registry_value(on_buf_enter)?),
            ..Default::default()
        })
    }
}

impl Augroup {
    /// TODO: docs
    pub fn clear_local(&self, lua: &Lua, bufnr: u16) -> LuaResult<()> {
        api::clear_autocmds(lua, lua.create_table_from([("buffer", bufnr)])?)
    }

    /// TODO: docs
    pub fn is_set(&self) -> bool {
        self.id.is_some()
    }

    /// Creates a new augroup, adding a global autocommand on the `BufEnter`
    /// event to attach to buffers. Should only be called if the augroup is not
    /// set.
    pub fn set(&mut self, lua: &Lua) -> LuaResult<()> {
        let id = api::create_augroup(
            lua,
            AUGROUP_NAME,
            lua.create_table_from([("clear", true)])?,
        )?;

        let on_buf_enter = lua.registry_value::<LuaFunction>(
            self.on_buf_enter_key.as_ref().expect("augroup already setup"),
        )?;

        api::create_autocmd(
            lua,
            ["BufEnter"],
            lua.create_table_from([
                ("group", LuaValue::Integer(id as i64)),
                ("callback", LuaValue::Function(on_buf_enter)),
            ])?,
        )?;

        self.id = Some(id);

        Ok(())
    }

    /// Add new buffer-local autocommands to the group. Should only be called
    /// if the augroup is set.
    pub fn set_local(
        &self,
        lua: &Lua,
        bufnr: u16,
        events: Vec<(&'static str, LuaFunction)>,
    ) -> LuaResult<()> {
        let id = self.id.expect("augroup is set");

        for (event, callback) in events {
            let opts = lua.create_table_from([
                ("group", LuaValue::Integer(id as i64)),
                ("callback", LuaValue::Function(callback)),
                ("buffer", LuaValue::Integer(bufnr as i64)),
            ])?;

            api::create_autocmd(lua, vec![event], opts)?;
        }

        Ok(())
    }

    /// TODO: docs
    pub fn unset(&mut self, lua: &Lua) -> LuaResult<()> {
        if let Some(id) = self.id {
            api::del_augroup_by_id(lua, id)?;
            self.id = None;
        }

        Ok(())
    }
}
