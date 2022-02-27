use mlua::Result;
use std::sync::{Arc, Mutex};

use crate::completion::CompletionState;
use crate::config::Config;
use crate::nvim::Nvim;
use crate::ui::UIState;

pub struct State {
    /// TODO: docs
    pub completion: Arc<Mutex<CompletionState>>,

    /// TODO: docs
    pub config: Arc<Mutex<Config>>,

    /// TODO: docs
    pub ui: Arc<Mutex<UIState>>,
}

impl State {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(State {
            completion: Arc::new(Mutex::new(CompletionState::new())),
            config: Arc::new(Mutex::new(Config::default())),
            ui: Arc::new(Mutex::new(UIState::new(nvim)?)),
        })
    }
}
