use mlua::prelude::LuaResult;
use neovim::Api;
use std::collections::HashMap;

use crate::completion::{CompletionItem, Cursor};
use crate::settings::Settings;
use crate::ui::Ui;

#[derive(Debug)]
pub struct State {
    /// Contains the buffer numbers of all the currently attached buffers.
    pub attached_buffers: Vec<u32>,

    /// TODO: docs
    pub bufenter_autocmd_id: Option<u32>,

    /// TODO: docs
    pub buffer_local_autocmds: HashMap<u32, Vec<u32>>,

    /// TODO: docs
    pub buffers_to_be_detached: Vec<u32>,

    /// The currently available completion items.
    pub completions: Vec<CompletionItem>,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// Whether the `require('compleet').setup` function has been called yet.
    pub did_setup: bool,

    /// Used to store the current configuration.
    pub settings: Settings,

    /// Holds state about the currently displayed UI.
    pub ui: Ui,
}

impl State {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(State {
            attached_buffers: Vec::new(),
            bufenter_autocmd_id: None,
            buffer_local_autocmds: HashMap::new(),
            buffers_to_be_detached: Vec::new(),
            completions: Vec::new(),
            cursor: Cursor::new(),
            did_setup: false,
            settings: Settings::default(),
            ui: Ui::new(api)?,
        })
    }
}
