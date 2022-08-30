use nvim::api::Buffer;
use nvim_oxi as nvim;

use super::{CompletionHint, CompletionItemDetails, CompletionMenu, UiConfig};

#[derive(Default)]
pub(crate) struct UiState {
    /// Virtual text displayed after the cursor to hint what characters would
    /// be inserted in the buffer if a completion was to be accepted.
    pub(crate) hint: CompletionHint,

    /// A floating window used to display the currently available completion
    /// items.
    pub(crate) menu: CompletionMenu,

    /// A floating window, usually displayed to the right or left of the
    /// completion menu, used to display additional informations (if any are
    /// available) about the currently selected completion item. For example,
    /// for a completion coming from the LSP source it might show documentation
    /// about a specific function.
    pub(crate) details: CompletionItemDetails,

    config: UiConfig,
}

impl UiState {
    #[inline]
    pub(crate) fn set_config(&mut self, config: UiConfig) {
        self.config = config;
    }

    #[inline]
    pub(crate) fn clean_all(&mut self, buf: &mut Buffer) -> nvim::Result<()> {
        self.hint.hide(buf)
    }
}
