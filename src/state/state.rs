use mlua::Result;
use std::sync::{Arc, Mutex};

use super::CompletionState;
use super::UIState;
use crate::config::Config;
use crate::nvim::Nvim;

pub struct State {
    /// Holds state about values used to compute the completion candidates.
    pub completion: Arc<Mutex<CompletionState>>,

    /// Used to store the current configuration.
    pub config: Arc<Mutex<Config>>,

    /// Holds state about the currently displayed UI.
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
