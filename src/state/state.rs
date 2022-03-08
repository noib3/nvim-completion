use mlua::Result;

use super::CompletionState;
use super::UIState;
use crate::nvim::Nvim;
use crate::settings::Settings;

pub struct State {
    /// Holds state about values used to compute the completion candidates.
    pub completion: CompletionState,

    /// Used to store the current configuration.
    pub settings: Settings,

    /// Holds state about the currently displayed UI.
    pub ui: UIState,
}

impl State {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(State {
            completion: CompletionState::new(),
            settings: Settings::default(),
            ui: UIState::new(nvim)?,
        })
    }
}
