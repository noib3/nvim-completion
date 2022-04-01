use mlua::prelude::{Lua, LuaRegistryKey, LuaResult};

use crate::channel::Channel;
use crate::settings::Settings;

pub struct State {
    pub augroup_id: Option<u32>,
    pub channel: Option<Channel>,
    pub did_setup: bool,
    pub settings: Settings,
    pub try_buf_attach: Option<LuaRegistryKey>,
}

impl State {
    pub fn new(_lua: &Lua) -> LuaResult<State> {
        Ok(State {
            augroup_id: None,
            channel: None,
            did_setup: false,
            settings: Settings::default(),
            try_buf_attach: None,
        })
    }
}
