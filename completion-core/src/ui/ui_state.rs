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

    /// The amount of total vertical space available for drawing our UI
    /// elements.
    ///
    /// Equal to the height of the terminal window minus (from top to bottom):
    ///
    /// - 1 if the tabline is visible (`:h showtabline`);
    /// - 1 if the statusline is visible (`:h laststatus`);
    /// - #rows used for the command-line (`:h cmdheight`);
    ///
    /// This is only updated on the `VimResized` event, which means if the user
    /// changes one of those options without also resizing the terminal this
    /// value will be outdated.
    rows: u16,

    /// The amount of total horizontal space available for drawing our UI
    /// elements.
    ///
    /// Always equal to the width of the terminal window. Like
    /// [`rows`](`UiState::rows`), this too is only updated on the `VimResized`
    /// event. However since it doesn't depend on any user-modifiable setting
    /// it should never get out of sync with its "right" value.
    columns: u16,
}

impl UiState {
    #[inline]
    pub(crate) fn init(
        &mut self,
        UiConfig { menu, details, hint }: UiConfig,
    ) -> nvim::Result<()> {
        self.hint.init(hint);
        self.menu.init(menu)?;
        self.details.init(details)?;

        self.update_rows()?;
        self.update_columns()?;

        Ok(())
    }

    #[inline]
    pub(crate) fn clean_all(&mut self, buf: &mut Buffer) -> nvim::Result<()> {
        self.hint.hide(buf)
    }

    #[inline]
    pub(crate) fn update_rows(&mut self) -> nvim::Result<()> {
        self.rows = super::utils::get_drawable_rows()?;
        Ok(())
    }

    #[inline]
    pub(crate) fn update_columns(&mut self) -> nvim::Result<()> {
        self.columns = super::utils::get_drawable_columns()?;
        Ok(())
    }
}
