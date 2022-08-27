use nvim::api::Buffer;
use nvim_oxi as nvim;

#[derive(Default)]
pub(crate) struct Ui {
    /// Virtual text displayed after the cursor to hint what characters would
    /// be inserted in the buffer if a completion was to be accepted.
    pub(crate) hint: super::CompletionHint,

    /// A floating window used to display the currently available completion
    /// items.
    pub(crate) menu: super::CompletionMenu,

    /// A floating window, usually displayed to the right or left of the
    /// completion menu, used to display additional informations (if any are
    /// available) about the currently selected completion item. For example,
    /// for a completion coming from the LSP source it might show documentation
    /// about a specific function.
    pub(crate) details: super::CompletionItemDetails,
}

impl Ui {
    pub(crate) fn clean_all(&mut self, buf: &mut Buffer) -> nvim::Result<()> {
        self.hint.hide(buf)
    }
}
