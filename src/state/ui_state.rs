use mlua::Result;
use neovim::Neovim;

use crate::ui::{CompletionHint, CompletionMenu, DetailsPane};

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
    pub fn new(nvim: &Neovim) -> Result<Self> {
        Ok(UIState {
            completion_menu: CompletionMenu::new(&nvim.api)?,
            completion_hint: CompletionHint::new(&nvim.api)?,
            details_pane: DetailsPane::new(&nvim.api)?,
        })
    }
}
