use compleet::completion::Completions;
use compleet::cursor::Cursor;
use mlua::prelude::LuaRegistryKey;

use crate::autocmds::Augroup;
use crate::channel::Channel;
use crate::settings::Settings;
use crate::ui::{Buffer, Ui};

#[derive(Debug, Default)]
pub struct State {
    /// The currently attached buffers.
    pub attached_buffers: Vec<Buffer>,

    /// The augroup namespacing all the autocmds.
    pub augroup: Augroup,

    /// A vector of buffers numbers to be detached on the next call to
    /// `on_bytes`.
    pub buffers_to_be_detached: Vec<u32>,

    /// The channel used to communicate with the server.
    pub channel: Channel,

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
    pub on_buf_enter_key: Option<LuaRegistryKey>,

    /// The current state of the UI.
    pub ui: Ui,
}
