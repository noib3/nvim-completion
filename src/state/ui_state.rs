use mlua::Result;

use crate::ui::{CompletionHint, CompletionMenu, DetailsPane};
use crate::Nvim;

/// `nvim-compleet`'s UI is composed of the following 3 independent pieces.
pub struct UIState {
    /// A completion menu used to show all the available completion candidates.
    pub completion_menu: CompletionMenu,

    /// A hint used to show the text that would be inserted in the buffer if
    /// the current completion item was accepted.
    pub completion_hint: CompletionHint,

    /// A details pane used to show some informations about the currently
    /// selected completion item.
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
