use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::RangeBounds;
use std::rc::Rc;
use std::sync::Arc;

use completion_types::{
    ClientMessage,
    ClientReceiver,
    ClientSender,
    Clock,
    CoreMessage,
    CoreSender,
    Document,
    Position,
    Revision,
};
use nvim::api::Buffer;
use nvim::opts::{BufAttachOpts, OnBytesArgs, ShouldDetach};
use nvim_oxi as nvim;

use crate::autocmds;
use crate::config::CompletionConfig;
use crate::lateinit::LateInit;
use crate::messages::echoerr;
use crate::ui::{self, UiConfig};
use crate::{DocumentExt, Error, PositionExt, Result};

thread_local! {
    /// TODO: docs
    static AUGROUP_ID: LateInit<u32> = LateInit::new();

    /// Message sender used to communicate with the callback executed on the
    /// main thread.
    static CLIENT_SENDER: LateInit<ClientSender> = LateInit::new();

    /// Message sender used to communicate with the thread pool where the
    /// completion results are computed.
    static CORE_SENDER: LateInit<CoreSender> = LateInit::new();
}

/// The client acts as a "central authority" connecting various parts of the
/// plugin together. It also holds the current state of the world and it's the
/// only entity able to read and modify it.
#[derive(Default, Clone)]
pub struct Client {
    // /// TODO: docs
    // augroup_id: LateInit<u32>,

    // /// Message sender used to communicate with the callback executed on the
    // /// main thread.
    // ui_sender: LateInit<UiSender>,

    // /// Message sender used to communicate with the thread pool where the
    // /// completion results are computed.
    // core_sender: LateInit<CoreSender>,
    state: Rc<RefCell<State>>,
}

#[derive(Default)]
struct State {
    /// TODO: docs
    completion_config: CompletionConfig,

    /// TODO: docs
    ui_config: UiConfig,

    /// TODO: docs
    documents: HashMap<Buffer, Arc<Document>>,

    /// TODO: docs
    revision: Revision,

    /// TODO: docs
    is_accepting_completions: bool,

    /// Virtual text displayed after the cursor to hint what characters would
    /// be inserted in the buffer if a completion was to be accepted.
    hint: ui::CompletionHint,

    /// A floating window used to display the currently available completion
    /// items.
    menu: ui::CompletionMenu,

    /// A floating window, usually displayed to the right or left of the
    /// completion menu, used to display additional informations (if any are
    /// available) about the currently selected completion item. For example,
    /// for a completion coming from the LSP source it might show documentation
    /// about a specific function.
    details: ui::CompletionItemDetails,

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
    /// [`rows`](`State::rows`), this too is only updated on the `VimResized`
    /// event. However since it doesn't depend on any user-modifiable setting
    /// it should never get out of sync with its "right" value.
    columns: u16,
}

impl Client {
    // Initialization

    pub(crate) fn init(
        &self,
        augroup_id: u32,
        ui_sender: ClientSender,
        core_sender: CoreSender,
        completion_config: CompletionConfig,
        ui_config: UiConfig,
    ) {
        AUGROUP_ID.with(|id| id.set(augroup_id));
        CLIENT_SENDER.with(|sender| sender.set(ui_sender));
        CORE_SENDER.with(|sender| sender.set(core_sender));
        // self.ui_sender.set(ui_sender);
        // self.core_sender.set(core_sender);

        let state = &mut *self.state.borrow_mut();
        state.completion_config = completion_config;
        state.ui_config = ui_config;
    }

    // Messages sent to the core.

    #[inline]
    fn send_core(&self, msg: CoreMessage) {
        CORE_SENDER.with(move |sender| sender.send(msg).unwrap());
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn query_attach(&self, buffer: Buffer) -> Result<()> {
        Document::new(buffer, CLIENT_SENDER.with(|sender| (**sender).clone()))
            .map(|document| CoreMessage::QueryAttach { document })
            .map(|msg| self.send_core(msg))
            .map_err(Into::into)
    }

    /// TODO: docs
    pub(crate) fn recompute_completions(
        &self,
        buffer: Buffer,
        position: Position,
        clock: Clock,
    ) {
        let state = &mut *self.state.borrow_mut();
        state.revision.advance();

        let document = state.documents.get(&buffer).map(Arc::clone).unwrap();

        let msg = CoreMessage::RecomputeCompletions {
            revision: state.revision,
            document,
            position,
            clock,
        };

        self.send_core(msg);
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn send_completions<R>(&self, range: R)
    where
        R: RangeBounds<u32>,
    {
        let msg = CoreMessage::SendCompletions {
            revision: self.state.borrow().revision,
            from: range.start_bound().cloned(),
            to: range.end_bound().cloned(),
        };

        self.send_core(msg);
    }

    /// Notifies the core to stop sending completion items for the current
    /// revision even if better results become available.
    #[inline]
    pub(crate) fn stop_sending(&self) {
        let msg = CoreMessage::StopSending {
            revision: self.state.borrow().revision,
        };

        self.send_core(msg);
    }

    // Messages coming from the core.

    fn on_bytes(
        &self,
        (
            _,
            buffer,
            changedtick,
            start_row,
            start_col,
            _byte_offset,
            rows_deleted,
            _cols_deleted,
            bytes_deleted,
            rows_added,
            _cols_added,
            bytes_added,
        ): OnBytesArgs,
    ) -> Result<ShouldDetach> {
        let clock = Clock::start();

        // If we've added or deleted a line we return early. If we've stayed on
        // the same line but we've deleted characters we only continue if the
        // `completion.while_deleting` option is set.
        if rows_added != 0
            || rows_deleted != 0
            || (bytes_deleted != 0 && !false/* TODO */)
        {
            return Ok(false);
        }

        // We only care about insert mode events.
        if !nvim::api::get_mode()?.mode.is_insert() {
            return Ok(false);
        }

        let col = start_col + if bytes_deleted != 0 { 0 } else { bytes_added };

        let position = Position::from_row_col_buf(start_row, col, &buffer)?;

        self.recompute_completions(buffer, position, clock);

        Ok(false)
    }

    pub(crate) fn handle_core_message(
        &self,
        receiver: &mut ClientReceiver,
    ) -> Result<()> {
        while let Ok(msg) = receiver.try_recv() {
            match msg {
                ClientMessage::AttachDocument { document } => {
                    self.attach_document(document)?
                },

                ClientMessage::ExecuteLuaFunction { fun } => {
                    nvim::schedule(move |_| fun(()))
                },

                ClientMessage::SourceEnableFailed { source, error } => {
                    return Err(Error::SourceEnableFailed {
                        sauce: source,
                        why: error.to_string(),
                    })
                },

                ClientMessage::SourceCompleteFailed { source, error } => {
                    return Err(Error::SourceCompleteFailed {
                        sauce: source,
                        why: error.to_string(),
                    })
                },

                ClientMessage::Completions {
                    items,
                    to,
                    from,
                    revision,
                    clock,
                } => {
                    nvim::schedule(move |_| {
                        nvim::print!(
                            "items: {:#?}, range: {:?}, revision: {:?}, \
                             clock: {:?}",
                            items,
                            (to, from),
                            revision,
                            clock
                        );

                        Ok(())
                    });
                },
            }
        }

        Ok(())
    }

    /// TODO: docs
    pub(crate) fn attach_document(
        &self,
        document: Arc<Document>,
    ) -> Result<()> {
        let state = &mut *self.state.borrow_mut();

        let buffer = document.buffer();

        let opts = {
            let on_bytes = self.to_nvim_fn(Self::on_bytes);
            BufAttachOpts::builder().on_bytes(on_bytes).build()
        };

        buffer.attach(false, &opts)?;
        autocmds::attach(self, AUGROUP_ID.with(|id| **id), buffer.clone())?;
        state.documents.insert(buffer, document);

        Ok(())
    }

    // Miscellaneous

    /// Converts a closure taking the client as the first argument to a
    /// [`nvim_oxi::Function`] with the same return type.
    ///
    /// If the closure returns an error we don't bubble it up unless it's an
    /// [`nvim_oxi::Error`]. We instead display an error message and return
    /// `R`'s default value.
    pub(crate) fn to_nvim_fn<F, A, R>(&self, fun: F) -> nvim::Function<A, R>
    where
        F: Fn(&Self, A) -> Result<R> + 'static,
        A: nvim::LuaPoppable,
        R: nvim::LuaPushable + Default,
    {
        let client = self.clone();

        nvim::Function::from_fn(move |args: A| match fun(&client, args) {
            Ok(r) => Ok(r),

            Err(Error::Nvim(nvim)) => Err(nvim),

            Err(other) => {
                echoerr!("{}", other);
                Ok(R::default())
            },
        })
    }
}
