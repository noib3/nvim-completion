use mlua::prelude::LuaResult;
use neovim::Api;

use crate::completion::{CompletionItem, Cursor};
use crate::settings::Settings;
use crate::ui::UI;

#[derive(Debug)]
pub struct State {
    /// TODO: docs
    pub augroup_id: Option<u32>,

    /// The currently available completion items computed by
    /// `completion::algo::complete`.
    pub completions: Vec<CompletionItem>,

    /// Holds state about the current cursor position.
    pub cursor: Cursor,

    /// Used to store the current configuration.
    pub settings: Settings,

    /// Holds state about the currently displayed UI.
    pub ui: UI,
}

impl State {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(State {
            augroup_id: None,
            completions: Vec::new(),
            cursor: Cursor::new(),
            settings: Settings::default(),
            ui: UI::new(api)?,
        })
    }
}
