use std::collections::HashMap;
use std::sync::Arc;

use bindings::opinionated::Buffer;
use sources::prelude::{Completions, Cursor};

use crate::{autocmds::Augroup, channel::Channel, settings::Settings, ui::Ui};

#[derive(Default)]
pub struct State {
    /// The currently attached buffers.
    pub attached_buffers: HashMap<u16, Arc<Buffer>>,

    /// The augroup namespacing all the autocommands.
    pub augroup: Augroup,

    // TODO: remove after https://github.com/neovim/neovim/issues/17874.
    buffers_to_be_detached: Vec<u16>,

    /// TODO: docs
    pub changedtick_last_seen: u32,

    /// TODO: docs
    pub changedtick_last_update: u32,

    /// A channel used to communicate w/ the tokio threadpool where the
    /// completion results are computed.
    pub channel: Option<Channel>,

    /// The currently available completion items.
    pub completions: Completions,

    /// Holds state about the cursor position in the current buffer.
    pub cursor: Cursor,

    /// Set to `true` right after `on_bytes` gets called.
    pub did_on_bytes: bool,

    /// TODO: docs.
    pub ignore_next_on_bytes: bool,

    /// Whether the setup function has ever been called.
    pub did_setup: bool,

    /// TODO: docs
    pub matched_bytes: usize,

    /// The current settings.
    pub settings: Settings,

    /// The current state of the UI.
    pub ui: Ui,
}

impl State {
    pub fn is_buffer_attached(&self, buffer: &Buffer) -> bool {
        self.attached_buffers.get(&buffer.bufnr).is_some()
    }

    pub fn cancel_detach_buffer(&mut self, buffer: &Buffer) {
        self.buffers_to_be_detached.retain(|&b| b != buffer.bufnr)
    }

    pub fn cancel_detach_all(&mut self) {
        self.buffers_to_be_detached.clear()
    }

    pub fn detach_buffer(&mut self, buffer: &Buffer) {
        self.attached_buffers.remove(&buffer.bufnr);
        self.buffers_to_be_detached.push(buffer.bufnr);
    }

    pub fn should_detach(&mut self, bufnr: u16) -> bool {
        if !self.buffers_to_be_detached.contains(&bufnr) {
            return false;
        }
        self.buffers_to_be_detached.retain(|&num| bufnr != num);
        true
    }

    pub fn attach_buffer(&mut self, buffer: Buffer) {
        self.attached_buffers.insert(buffer.bufnr, Arc::new(buffer));
    }

    pub fn _get_buffer(&self, bufnr: u16) -> Option<Arc<Buffer>> {
        self.attached_buffers.get(&bufnr).map(|buf| buf.clone())
    }

    pub fn detach_all_buffers(&mut self) {
        self.buffers_to_be_detached.extend(
            self.attached_buffers.drain().map(|(bufnr, _buffer)| bufnr),
        );
    }
}
