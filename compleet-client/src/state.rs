use std::collections::HashMap;

use compleet::completion::Completions;
use compleet::cursor::Cursor;
use mlua::prelude::{Lua, LuaRegistryKey, LuaResult};

use crate::channel::Channel;
use crate::settings::Settings;
use crate::ui::Ui;

#[derive(Debug)]
pub struct State {
    /// The buffer numbers of the currently attached buffers.
    pub attached_buffers: Vec<u32>,

    /// The id of the `Compleet` augroup.
    pub augroup_id: Option<u32>,

    /// A hashmap where the keys are the numbers of the currently attached
    /// buffers and the values are the ids of the autocommands registered on
    /// that buffer.
    pub buffer_local_autocmds: HashMap<u32, Vec<u32>>,

    /// A vector of buffers numbers to be detached on the next call to
    /// `completion::on_bytes`.
    pub buffers_to_be_detached: Vec<u32>,

    /// The channel used to communicate with the server.
    pub channel: Option<Channel>,

    /// The currently available completion items.
    pub completions: Completions,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// Set to `true` right after `on_bytes` gets called.
    pub did_on_bytes: bool,

    /// Whether the setup function has ever been called.
    pub did_setup: bool,

    /// The current settings.
    pub settings: Settings,

    /// The registry key of the Lua function called on `BufEnter` to try to
    /// attach to the buffer.
    pub try_buf_attach: Option<LuaRegistryKey>,

    /// The current state of the UI.
    pub ui: Option<Ui>,
}

impl State {
    pub fn new(lua: &Lua) -> LuaResult<State> {
        Ok(State {
            attached_buffers: Vec::new(),
            augroup_id: None,
            channel: None,
            buffer_local_autocmds: HashMap::new(),
            buffers_to_be_detached: Vec::new(),
            completions: Vec::new(),
            cursor: Cursor::new(),
            did_on_bytes: false,
            did_setup: false,
            settings: Settings::default(),
            try_buf_attach: None,
            ui: None,
        })
    }
}
