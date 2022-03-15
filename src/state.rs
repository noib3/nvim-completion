use mlua::Result;
use neovim::Neovim;

use crate::completion::{CompletionItem, Line};
use crate::settings::Settings;
use crate::ui::UI;

#[derive(Debug)]
pub struct State {
    /// TODO: docs
    pub augroup_id: Option<usize>,

    /// The currently available completion items computed by
    /// `completion::algo::complete`.
    pub completions: Vec<CompletionItem>,

    /// Holds state about values used to compute the completion candidates.
    pub line: Line,

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
            line: Line::new(),
            settings: Settings::default(),
            ui: UI::new(nvim)?,
        })
    }
}
