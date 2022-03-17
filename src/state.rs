use mlua::Result;
use neovim::Neovim;

use crate::completion::{Buffer, CompletionItem};
use crate::settings::Settings;
use crate::ui::UI;

#[derive(Debug)]
pub struct State {
    /// TODO: docs
    pub augroup_id: Option<usize>,

    /// The currently available completion items computed by
    /// `completion::algo::complete`.
    pub completions: Vec<CompletionItem>,

    /// Holds state about the current buffer.
    pub buffer: Buffer,

    /// Used to store the current configuration.
    pub settings: Settings,

    /// Holds state about the currently displayed UI.
    pub ui: UI,
}

impl State {
    pub fn new(nvim: &Neovim) -> Result<Self> {
        Ok(State {
            augroup_id: None,
            completions: Vec::new(),
            buffer: Buffer::new(),
            settings: Settings::default(),
            ui: UI::new(nvim)?,
        })
    }
}
