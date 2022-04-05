use std::collections::HashMap;

use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaValue, ToLua};

use crate::bindings::api;
use crate::constants::AUGROUP_NAME;
use crate::ui::Buffer;

#[derive(Debug, Default)]
pub struct Augroup {
    /// The augroup id returned by `vim.api.nvim_create_autocmd`.
    id: Option<u32>,

    /// The ids of the global autocmds, i.e. the ones registered on every
    /// buffer.
    global_autocmds: Vec<u32>,

    /// The keys are buffer numbers and the values are the ids of all the
    /// autocommands registered on that buffer.
    buflocal_autocmds: HashMap<u32, Vec<u32>>,
}

impl Augroup {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        let opts = lua.create_table_from([("clear", true)])?;

        Ok(Augroup {
            id: Some(api::create_augroup(lua, AUGROUP_NAME, opts)?),
            ..Default::default()
        })
    }
}

impl Augroup {
    /// Add new autocommands to the group.
    pub fn add_autocmds(
        &mut self,
        lua: &Lua,
        buffer: Option<&Buffer>,
        events: Vec<(&'static str, LuaFunction)>,
    ) -> LuaResult<()> {
        let id = self.id.expect("I'm adding autocmds so it must be set");

        let ids = events
            .into_iter()
            .flat_map(|(event, callback)| {
                let opts = lua.create_table_from([
                    ("group", LuaValue::Integer(id as i64)),
                    ("callback", LuaValue::Function(callback)),
                    ("buffer", buffer.map(|b| b.number).to_lua(lua)?),
                ])?;

                api::create_autocmd(lua, vec![event], opts)
            })
            .collect::<Vec<u32>>();

        self.global_autocmds.extend(ids);

        Ok(())
    }

    /// TODO: docs
    pub fn delete_all(&mut self, lua: &Lua) -> LuaResult<()> {
        if let Some(id) = self.id {
            api::del_augroup_by_id(lua, id)?;
            self.id = None;
            self.global_autocmds.clear();
            self.buflocal_autocmds.clear();
        }

        Ok(())
    }

    /// TODO: docs
    pub fn delete_local(
        &mut self,
        lua: &Lua,
        buffer: &Buffer,
    ) -> LuaResult<()> {
        if let Some((_, ids)) =
            self.buflocal_autocmds.remove_entry(&buffer.number)
        {
            for id in ids {
                api::del_autocmd(lua, id)?;
            }
        }

        Ok(())
    }

    /// TODO: docs
    pub fn is_active(&self) -> bool {
        self.id.is_some()
    }

    /// TODO: docs
    pub fn is_buf_enter_set(&self) -> bool {
        !self.global_autocmds.is_empty()
    }
}
