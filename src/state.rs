use mlua::Result;
use std::sync::{Arc, Mutex};

use crate::completion::CompletionState;
use crate::nvim::Nvim;
use crate::ui::UIState;

pub struct State {
    /// TODO: docs
    pub completion: Arc<Mutex<CompletionState>>,

    /// TODO: docs
    pub ui: Arc<Mutex<UIState>>,
}

impl State {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(State {
            completion: Arc::new(Mutex::new(CompletionState::new())),
            ui: Arc::new(Mutex::new(UIState::new(nvim)?)),
        })
    }
}
