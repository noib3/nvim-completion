use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;

use completion_types::{
    ClientMessage,
    ClientSender,
    Clock,
    CoreMessage,
    CoreReceiver,
    CoreSender,
    Document,
    Position,
    Revision,
    ScoredCompletion,
};
use nvim::api::Buffer;
use nvim::opts::{BufAttachOpts, OnBytesArgs, ShouldDetach};
use nvim_oxi as nvim;

use crate::autocmds;
use crate::config::CompletionConfig;
use crate::lateinit::LateInit;
use crate::messages::echoerr;
use crate::ui::{UiConfig, UiState};
use crate::{DocumentExt, Error, PositionExt, Result};

thread_local! {
    /// TODO: docs
    static AUGROUP_ID: LateInit<u32> = LateInit::new();

    /// Message sender used to communicate with the callback executed on the
    /// main thread.
    static CLIENT_SENDER: LateInit<CoreSender> = LateInit::new();

    /// Message sender used to communicate with the thread pool where the
    /// completion results are computed.
    static CORE_SENDER: LateInit<ClientSender> = LateInit::new();
}

/// The client acts as a "central authority" connecting various parts of the
/// plugin together. It also holds the current state of the world and it's the
/// only entity able to read and modify it.
#[derive(Default, Clone)]
pub struct Client {
    state: Rc<RefCell<State>>,
    ui_state: Rc<RefCell<UiState>>,
}

#[derive(Default)]
struct State {
    /// TODO: docs
    completion_config: CompletionConfig,

    /// TODO: docs
    documents: HashMap<Buffer, Arc<Document>>,

    /// TODO: docs
    revision: Revision,

    /// TODO: docs
    is_accepting_completions: bool,
}

impl Client {
    // Initialization

    pub(crate) fn init(
        &self,
        augroup_id: u32,
        ui_sender: CoreSender,
        core_sender: ClientSender,
        completion_config: CompletionConfig,
        ui_config: UiConfig,
    ) -> nvim::Result<()> {
        AUGROUP_ID.with(|id| id.set(augroup_id));
        CLIENT_SENDER.with(|sender| sender.set(ui_sender));
        CORE_SENDER.with(|sender| sender.set(core_sender));

        let state = &mut *self.state.borrow_mut();
        state.completion_config = completion_config;

        let ui_state = &mut *self.ui_state.borrow_mut();
        ui_state.init(ui_config)
    }

    // Messages sent to the core.

    #[inline]
    fn send_core(&self, msg: ClientMessage) {
        CORE_SENDER.with(move |sender| sender.send(msg).unwrap());
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn query_attach(&self, buffer: Buffer) -> Result<()> {
        Document::new(buffer, CLIENT_SENDER.with(|sender| (**sender).clone()))
            .map(|document| ClientMessage::QueryAttach { document })
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
        state.is_accepting_completions = true;

        let document = state.documents.get(&buffer).map(Arc::clone).unwrap();

        let msg = ClientMessage::RecomputeCompletions {
            revision: state.revision,
            document,
            position,
            clock,
        };

        self.send_core(msg);
    }

    /// Notifies the core to stop sending completion items for the current
    /// revision even if better results become available.
    #[inline]
    pub(crate) fn stop_sending(&self) {
        let state = &mut *self.state.borrow_mut();
        state.is_accepting_completions = false;

        let msg = ClientMessage::StopSending { revision: state.revision };

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
        receiver: &mut CoreReceiver,
    ) -> Result<()> {
        let mut completions = None;

        while let Ok(msg) = receiver.try_recv() {
            match msg {
                CoreMessage::AttachDocument { document } => {
                    self.attach_document(document)?
                },

                CoreMessage::ExecuteLuaFunction { fun } => {
                    nvim::schedule(move |_| fun(()))
                },

                CoreMessage::SourceEnableFailed { source, error } => {
                    return Err(Error::SourceEnableFailed {
                        sauce: source,
                        why: error.to_string(),
                    })
                },

                CoreMessage::SourceCompleteFailed { source, error } => {
                    return Err(Error::SourceCompleteFailed {
                        sauce: source,
                        why: error.to_string(),
                    })
                },

                CoreMessage::Completions {
                    items,
                    revision,
                    buffer,
                    position,
                    clock,
                } => {
                    if self.is_last_revision(revision) && !items.is_empty() {
                        completions = Some((items, buffer, position, clock));
                    }
                },
            }
        }

        if let Some((items, buffer, position, clock)) = completions {
            let client = self.clone();
            nvim::schedule(move |_| {
                client.update_completions(items, buffer, position, clock)
            })
        }

        Ok(())
    }

    fn is_last_revision(&self, revision: Revision) -> bool {
        self.state.borrow().revision == revision
    }

    fn update_completions(
        &self,
        completions: Vec<ScoredCompletion>,
        buffer: Buffer,
        position: Arc<Position>,
        mut clock: Clock,
    ) -> nvim::Result<()> {
        if self.state.borrow().is_accepting_completions {
            let ui_state = &mut *self.ui_state.borrow_mut();
            ui_state.update_completions(completions, buffer, position)?;
        }

        clock.time_displaying();
        nvim::print!("{:?}", clock);

        Ok(())
    }

    /// Gives mutable access to the UI state.
    pub(crate) fn ui_mut(&self) -> impl DerefMut<Target = UiState> + '_ {
        RefMut::map(self.ui_state.borrow_mut(), |state| state)
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
