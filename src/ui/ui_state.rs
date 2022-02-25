use mlua::Result;

use super::{CompletionMenu, DetailsPane, VirtualText};
use crate::Nvim;

pub struct UIState {
    /// TODO: docs
    pub completion_menu: CompletionMenu,

    /// TODO: docs
    pub details_pane: DetailsPane,

    /// TODO: docs
    pub virtual_text: VirtualText,
}

impl UIState {
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(UIState {
            completion_menu: CompletionMenu::new(nvim)?,
            details_pane: DetailsPane::new(nvim)?,
            virtual_text: VirtualText::new(),
        })
    }
}
