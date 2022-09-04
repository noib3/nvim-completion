use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use nvim::api::Buffer as NvimBuffer;
use nvim::opts::BufAttachOpts;
use nvim_oxi as nvim;

use crate::completions::{
    CompletionContext,
    CompletionRequest,
    CompletionState,
    RevId,
};
use crate::lateinit::LateInit;
use crate::messages::echoerr;
use crate::pipeline::{MainMessage, MainSender, PoolMessage, PoolSender};
use crate::ui::UiState;
use crate::{Buffer, Error, Result};

thread_local! {
    static AUGROUP_ID: LateInit<u32> = LateInit::new();

    /// Message sender used to communicate with the callback executed on the
    /// main thread.
    static MAIN_SENDER: LateInit<MainSender> = LateInit::new();

    /// Message sender used to communicate with the thread pool where the
    /// completion results are computed.
    static POOL_SENDER: LateInit<PoolSender> = LateInit::new();
}

/// The client acts as a "central authority" connecting various parts of the
/// plugin together. It also holds the current state of the world and it's the
/// only entity able to read and modify it.
#[derive(Default, Clone)]
pub struct Client {
    completion_state: Rc<RefCell<CompletionState>>,
    misc_state: Rc<RefCell<MiscState>>,
    ui_state: Rc<RefCell<UiState>>,
}

#[derive(Default)]
struct MiscState {
    /// Map of attached buffers from [`nvim_oxi`]'s `Buffer`s to the `Buffer`
    /// type defined in this crate. A buffer is considered "attached" if one or
    /// more completion sources have attached to it.
    //
    // REFACTOR: this should be rethought.
    bufs: HashMap<NvimBuffer, Arc<Buffer>>,

    /// An identifier for the last edit in one of the attached buffers.
    rev_id: RevId,
}

impl Client {
    /// Gives mutable access to the completion state.
    pub(crate) fn completion(
        &self,
    ) -> impl DerefMut<Target = CompletionState> + '_ {
        RefMut::map(self.completion_state.borrow_mut(), |state| state)
    }

    /// Gives mutable access to the UI state.
    pub(crate) fn ui(&self) -> impl DerefMut<Target = UiState> + '_ {
        RefMut::map(self.ui_state.borrow_mut(), |state| state)
    }

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

    // -----------------------------------------------------------------------
    // Setters.

    #[inline]
    pub(crate) fn set_augroup_id(&self, id: u32) {
        AUGROUP_ID.with(|augroup_id| augroup_id.set(id));
    }

    #[inline]
    pub(crate) fn set_main_sender(&self, sender: MainSender) {
        MAIN_SENDER.with(|main_sender| main_sender.set(sender));
    }

    #[inline]
    pub(crate) fn set_pool_sender(&self, sender: PoolSender) {
        POOL_SENDER.with(|pool_sender| pool_sender.set(sender));
    }

    // -----------------------------------------------------------------------
    // Thread pool messaging.

    #[inline]
    fn send_pool(&self, msg: PoolMessage) {
        POOL_SENDER.with(move |sender| sender.send(msg).unwrap());
    }

    #[inline]
    pub(crate) fn send_main(msg: MainMessage) {
        MAIN_SENDER.with(move |sender| sender.send(msg));
    }

    // TODO: docs
    #[inline]
    pub(crate) fn query_attach(&self, buf: NvimBuffer) -> Result<()> {
        let buf = Buffer::new(buf).map(Arc::new)?;
        self.send_pool(PoolMessage::QueryAttach(buf));
        Ok(())
    }

    // TODO: docs
    #[inline]
    pub(crate) fn query_completions(
        &self,
        buf: &NvimBuffer,
        ctx: CompletionContext,
        start: Instant,
        rev: RevId,
    ) {
        // TODO: explain why we can unwrap here.
        let buf =
            self.misc_state.borrow().bufs.get(buf).map(Arc::clone).unwrap();

        let req = CompletionRequest { buf, ctx, start, rev };

        self.send_pool(PoolMessage::QueryCompletions(Arc::new(req)))
    }

    /// Sends a message to the thread pool to stop any running tasks querying
    /// completion results from an earlier request.
    #[inline]
    pub(crate) fn stop_sources(&self) {
        self.send_pool(PoolMessage::AbortAll);
    }

    // -----------------------------------------------------------------------
    // Misc.

    /// Attaches a buffer by...
    pub(crate) fn attach_buffer(&self, buf: Arc<Buffer>) -> Result<()> {
        let nvim_buf = buf.nvim_buf();

        let on_bytes = self.to_nvim_fn(crate::pipeline::on_bytes);
        let opts = BufAttachOpts::builder().on_bytes(on_bytes).build();
        nvim_buf.attach(false, &opts)?;

        let state = &mut *self.misc_state.borrow_mut();
        crate::autocmds::attach_to_buffer(
            self,
            AUGROUP_ID.with(|id| **id),
            nvim_buf.clone(),
        )?;
        state.bufs.insert(nvim_buf, buf);

        Ok(())
    }

    /// Updates the last revision seen by the client.
    pub(crate) fn set_rev_id(&self, rev_id: RevId) {
        (*self.misc_state.borrow_mut()).rev_id = rev_id;
    }

    /// Returns whether the provided revision is the last one seen by the
    /// client.
    pub(crate) fn is_last_rev(&self, rev_id: &RevId) -> bool {
        &(*self.misc_state.borrow()).rev_id == rev_id
    }
}
