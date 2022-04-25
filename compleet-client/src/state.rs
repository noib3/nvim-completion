use sources::prelude::{Completions, Cursor};

use crate::{
    autocmds::Augroup,
    channel::Channel,
    settings::Settings,
    ui::{Buffer, Ui},
};

#[derive(Default)]
pub struct State {
    /// The currently attached buffers.
    pub attached_buffers: Vec<Buffer>,

    /// The augroup namespacing all the autocommands.
    pub augroup: Augroup,

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    pub buffers_to_be_detached: Vec<u16>,

    /// TODO: docs
    pub changedtick_last_seen: u32,

    /// TODO: docs
    pub changedtick_last_update: u32,

    /// A channel used to communicate w/ the tokio threadpool where completion
    /// results are computed.
    pub channel: Option<Channel>,

    /// The currently available completion items.
    pub completions: Completions,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// Set to `true` right after `on_bytes` gets called.
    pub did_on_bytes: bool,

    /// Whether the setup function has ever been called.
    pub did_setup: bool,

    /// TODO: docs
    pub matched_bytes: usize,

    /// The current settings.
    pub settings: Settings,

    /// The current state of the UI.
    pub ui: Ui,
}
