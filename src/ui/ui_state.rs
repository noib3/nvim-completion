use mlua::Result;

use super::{CompletionHint, CompletionMenu, DetailsPane};
use crate::Nvim;

pub struct UIState {
    /// TODO: docs
    pub completion_menu: CompletionMenu,

    /// TODO: docs
    pub completion_hint: CompletionHint,

    /// TODO: docs
    pub details_pane: DetailsPane,
}

impl UIState {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(UIState {
            completion_menu: CompletionMenu::new(nvim)?,
            completion_hint: CompletionHint::new(nvim)?,
            details_pane: DetailsPane::new(nvim)?,
        })
    }
}
