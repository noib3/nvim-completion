use std::collections::HashMap;
use std::sync::Arc;

use mlua::prelude::{LuaRegistryKey, LuaResult};
use neovim::Api;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::completion::{Completions, Cursor, Sources};
use crate::settings::Settings;
use crate::ui::Ui;

#[derive(Debug)]
pub struct State {
    /// Contains the buffer numbers of all the currently attached buffers.
    pub attached_buffers: Vec<u32>,

    /// The id of the `Compleet` augroup, or `None` if it isn't set.
    pub augroup_id: Option<u32>,

    /// A hashmap where the keys are the numbers of the currently attached
    /// buffers and the values are the ids of the autocommands registered on
    /// that buffer.
    pub buffer_local_autocmds: HashMap<u32, Vec<u32>>,

    /// A vector of buffers numbers to be detached on the next call to
    /// `completion::on_bytes`.
    pub buffers_to_be_detached: Vec<u32>,

    /// The currently available completion items.
    pub completions: Completions,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// TODO: docs
    pub did_on_bytes: bool,

    /// Whether the `require('compleet').setup` function has been called yet.
    pub did_setup: bool,

    /// TODO
    pub handles: Vec<JoinHandle<()>>,

    /// The async runtime used to get completions from sources.
    pub runtime: Option<Runtime>,

    /// TODO
    pub rx: Option<Receiver<Completions>>,

    /// TODO
    pub tx: Option<Arc<Sender<(Arc<Cursor>, Completions)>>>,

    /// Used to store the current configuration.
    pub settings: Settings,

    /// A hashmap where the keys are the numbers of the currently attached
    /// buffers and the values are the completion sources enabled in that
    /// buffer.
    pub sources: HashMap<u32, Sources>,

    /// A registry key pointing to the `try_buf_attach` Lua function used to
    /// attach to new buffers.
    pub try_buf_attach: Option<LuaRegistryKey>,

    /// Holds state about the currently displayed UI.
    pub ui: Ui,
}

impl State {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(State {
            attached_buffers: Vec::new(),
            augroup_id: None,
            buffer_local_autocmds: HashMap::new(),
            buffers_to_be_detached: Vec::new(),
            completions: Vec::new(),
            cursor: Cursor::new(),
            did_on_bytes: false,
            did_setup: false,
            handles: Vec::new(),
            runtime: None,
            rx: None,
            tx: None,
            settings: Settings::default(),
            sources: HashMap::new(),
            try_buf_attach: None,
            ui: Ui::new(api)?,
        })
    }
}
