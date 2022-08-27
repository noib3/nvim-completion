use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use nvim::api::Buffer as NvimBuffer;
use nvim_oxi::{
    self as nvim,
    opts::*,
    Dictionary,
    Function,
    LuaPoppable,
    LuaPushable,
    Object,
};
use tokio::sync::mpsc;

use crate::completion_bundle::{CompletionRequest, RevId};
use crate::completion_source::SourceId;
use crate::config::{Config, SOURCE_NAMES};
use crate::lateinit::LateInit;
use crate::threads::{self, MainMessage, PoolMessage};
use crate::{mappings, messages};
use crate::{
    Buffer,
    CompletionContext,
    CompletionItem,
    CompletionSource,
    Error,
};

#[derive(Default)]
/// The client acts as a "central authority" connecting various parts of the
/// plugin together. It also holds the current state of the world and it's the
/// only entity able to read and modify it.
pub struct Client {
    state: Rc<RefCell<State>>,
}

#[derive(Default)]
struct State {
    /// The id of the `Compleet` augroup if currently set, `None` otherwise.
    augroup_id: Option<u32>,

    /// Map of attached buffers from [`nvim_oxi`]'s `Buffer`s to the `Buffer`
    /// type defined in this crate. A buffer is considered "attached" if one or
    /// more completion sources have attached to it.
    bufs: HashMap<nvim::api::Buffer, Arc<crate::Buffer>>,

    /// Message sender used to communicate with the callback executed on the
    /// main thread.
    cb_sender: LateInit<mpsc::UnboundedSender<MainMessage>>,

    /// The current list of available completion items.
    // TODO: change to a BTree?
    completions: Vec<(CompletionItem, bool)>,

    /// Message sender used to communicate with the thread pool where the
    /// completion results are computed.
    pool_sender: LateInit<mpsc::UnboundedSender<PoolMessage>>,

    /// Whether the [`setup`](crate::setup) function has ever been called.
    did_setup: bool,

    /// An identifier for the last edit in one of the attached buffers.
    rev_id: RevId,

    /// Map containing all the sources registered via
    /// [`Client::register_source`]. The map keys are the source names, i.e.
    /// the output of [`CompletionSource::name`].
    sources: HashMap<&'static str, Arc<dyn CompletionSource>>,

    /// Map containing..
    source_stats: HashMap<SourceId, [u16; 1024]>,

    /// The UI of the plugin.
    ui: crate::ui::Ui,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self { state: Rc::clone(&self.state) }
    }
}

impl Client {
    // -----------------------------------------------------------------------
    // Public API.

    /// Creates a new [`Client`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    pub fn register_source<S>(&self, source: S)
    where
        S: CompletionSource,
    {
        SOURCE_NAMES.with(|names| {
            names.borrow_mut().as_mut().unwrap().push(source.name())
        });
        let sources = &mut self.state.borrow_mut().sources;
        sources.insert(source.name(), Arc::new(source));
    }

    /// Returns a [`Dictionary`] representing the public API of the plugin.
    pub fn build_api(&self) -> Dictionary {
        [("setup", Object::from(self.create_fn(crate::setup)))]
            .into_iter()
            .chain(mappings::setup(self))
            .chain(
                self.state
                    .borrow()
                    .sources
                    .iter()
                    .map(|(&name, source)| (name, source.api())),
            )
            .collect()
    }

    // -----------------------------------------------------------------------
    // Thread pool messaging.

    /// Sends a message to the thread pool.
    #[inline]
    fn send_pool_msg(&self, msg: PoolMessage) {
        self.state.borrow().pool_sender.send(msg).unwrap();
    }

    // TODO: docs
    #[inline]
    pub(crate) fn query_attach(&self, buf: NvimBuffer) -> crate::Result<()> {
        let buf = {
            let state = &*self.state.borrow();
            let sender = (*state.cb_sender).clone();
            crate::Buffer::new(buf, sender).map(Arc::new)?
        };
        self.send_pool_msg(PoolMessage::QueryAttach(buf));
        Ok(())
    }

    // TODO: docs
    #[inline]
    pub(crate) fn query_completions(
        &self,
        buf: &NvimBuffer,
        ctx: CompletionContext,
        start: Instant,
        ct: u32,
    ) {
        // TODO: explain why we can unwrap here.
        let buf = self.state.borrow().bufs.get(buf).map(Arc::clone).unwrap();
        let req = CompletionRequest::new(buf, ctx, start, ct);
        self.send_pool_msg(PoolMessage::QueryCompletions(Arc::new(req)))
    }

    /// Sends a message to the thread pool to stop any running tasks querying
    /// completion results from an earlier request.
    #[inline]
    pub(crate) fn stop_sources(&self) {
        self.send_pool_msg(PoolMessage::AbortAll);
    }

    // -----------------------------------------------------------------------
    // Misc.

    /// Attaches a buffer by...
    pub(crate) fn attach_buffer(&self, buf: Arc<Buffer>) -> crate::Result<()> {
        // Tell Neovim to call the [on_bytes] function on this buffer whenever
        // its contents are modified.
        let on_bytes = self.create_fn(crate::on_bytes);
        let opts = BufAttachOpts::builder().on_bytes(on_bytes).build();
        buf.nvim_buf().attach(false, &opts)?;

        // Add the buffer to the [bufs](Client::bufs) map.
        let state = &mut *self.state.borrow_mut();
        state.bufs.insert(buf.nvim_buf().clone(), buf);

        Ok(())
    }

    /// Returns whether the [setup] function has already been called.
    #[inline]
    pub(crate) fn already_setup(&self) -> bool {
        self.state.borrow().did_setup
    }

    pub(crate) fn create_fn<F, A, R, E>(&self, fun: F) -> Function<A, R>
    where
        F: Fn(&Self, A) -> Result<R, E> + 'static,
        A: LuaPoppable,
        R: LuaPushable + Default,
        E: Into<Error>,
    {
        let mut client = self.clone();
        Function::from_fn_mut(move |args| {
            match fun(&mut client, args).map_err(Into::into) {
                Ok(ret) => Ok(ret),

                Err(err) => match err {
                    Error::NvimError(nvim) => Err(nvim),

                    other => {
                        messages::echoerr!("{other}");
                        Ok(R::default())
                    },
                },
            }
        })
    }

    #[inline]
    pub(crate) fn did_setup(&self) {
        self.state.borrow_mut().did_setup = true;
    }

    pub(crate) fn set_config(&self, config: Config) {
        let state = &mut *self.state.borrow_mut();
        state.sources.retain(|name, _| {
            config.sources.get(*name).map(|enable| *enable).unwrap_or_default()
        });
    }

    pub(crate) fn start_channel(&self) -> crate::Result<()> {
        let (cb_sender, cb_receiver) = mpsc::unbounded_channel();
        let (pool_sender, pool_receiver) = mpsc::unbounded_channel();

        let state = &mut *self.state.borrow_mut();
        state.cb_sender.set(cb_sender.clone());
        state.pool_sender.set(pool_sender);

        let sources =
            state.sources.values().map(Arc::clone).collect::<Vec<_>>();

        threads::setup(
            self.clone(),
            sources,
            cb_sender,
            cb_receiver,
            pool_receiver,
        )
    }

    /// Updates the last revision seen by the client.
    pub(crate) fn set_rev_id(&self, rev_id: RevId) {
        (*self.state.borrow_mut()).rev_id = rev_id;
    }

    /// Returns whether the provided revision is the last one seen by the
    /// client.
    pub(crate) fn is_last_rev(&self, rev_id: &RevId) -> bool {
        &(*self.state.borrow()).rev_id == rev_id
    }
}
