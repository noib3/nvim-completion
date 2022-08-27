#[derive(Default)]
pub(crate) struct Ui {
    /// Virtual text displayed after the cursor to hint what characters would
    /// be inserted in the buffer if a completion was to be accepted.
    hint: super::CompletionHint,

    /// A floating window used to display the currently available completion
    /// items.
    menu: super::CompletionMenu,

    /// A floating window, usually displayed to the right or left of the
    /// completion menu, used to display additional informations (if any are
    /// available) about the currently selected completion item. For example,
    /// for a completion coming from the LSP source it might show documentation
    /// about a specific function.
    details: super::CompletionItemDetails,
}
