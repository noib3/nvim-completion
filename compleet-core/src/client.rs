use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use nvim::api::Buffer as NvimBuffer;
use nvim::r#loop::AsyncHandle;
use nvim_oxi::{
    self as nvim,
    api,
    opts::*,
    Dictionary,
    Function,
    LuaPoppable,
    LuaPushable,
    Object,
};
use nvim_oxi::{object, ObjectKind};
use tokio::sync::mpsc;

use crate::completions::{
    CompletionContext,
    CompletionRequest,
    RevId,
    SourceId,
};
use crate::config::{Config, SOURCE_NAMES};
use crate::lateinit::LateInit;
use crate::messages::echoerr;
use crate::pipeline::{self, MainSender, PoolMessage, PoolSender};
use crate::ui::Ui;
use crate::{commands, hlgroups};
use crate::{mappings, messages};
use crate::{Buffer, CompletionItem, CompletionSource, Error};

#[derive(Default)]
/// The client acts as a "central authority" connecting various parts of the
/// plugin together. It also holds the current state of the world and it's the
/// only entity able to read and modify it.
pub struct Client {
    state: Rc<RefCell<State>>,
}

#[derive(Default)]
struct State {
    /// The id of the `Compleet` augroup.
    augroup_id: LateInit<u32>,

    /// Map of attached buffers from [`nvim_oxi`]'s `Buffer`s to the `Buffer`
    /// type defined in this crate. A buffer is considered "attached" if one or
    /// more completion sources have attached to it.
    //
    // REFACTOR: this should be rethought.
    bufs: HashMap<nvim::api::Buffer, Arc<crate::Buffer>>,

    /// Message sender used to communicate with the callback executed on the
    /// main thread.
    main_sender: LateInit<MainSender>,

    /// The current list of available completion items.
    // TODO: change to a BTree?
    completions: Vec<(CompletionItem, bool)>,

    config: Config,

    /// Message sender used to communicate with the thread pool where the
    /// completion results are computed.
    pool_sender: LateInit<PoolSender>,

    /// Whether the [`setup`](crate::setup) function has ever been called.
    did_setup: bool,

    /// An identifier for the last edit in one of the attached buffers.
    rev_id: RevId,

    /// Map containing all the sources registered via
    /// [`Client::register_source`]. The map keys are the source names, i.e.
    /// the output of [`CompletionSource::name`].
    //
    // REFACTOR: this should be rethought. Do we need them here?
    sources: HashMap<&'static str, Arc<dyn CompletionSource>>,

    /// Map containing..
    source_stats: HashMap<SourceId, [u16; 1024]>,

    /// The UI of the plugin.
    ui: Ui,
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
    #[inline(always)]
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
        [("setup", Object::from(self.create_fn(Self::setup)))]
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
            let sender = (*self.state.borrow().main_sender).clone();
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
        rev: RevId,
    ) {
        // TODO: explain why we can unwrap here.
        let buf = self.state.borrow().bufs.get(buf).map(Arc::clone).unwrap();
        let req = CompletionRequest { buf, ctx, start, rev };
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
        let nvim_buf = buf.nvim_buf();

        let on_bytes = self.create_fn(pipeline::on_bytes);
        let opts = BufAttachOpts::builder().on_bytes(on_bytes).build();
        nvim_buf.attach(false, &opts)?;

        let state = &mut *self.state.borrow_mut();
        crate::autocmds::attach_to_buffer(
            self,
            *state.augroup_id,
            nvim_buf.clone(),
        )?;
        state.bufs.insert(nvim_buf, buf);

        Ok(())
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
                        // REFACTOR
                        messages::echoerr!("{other}");
                        Ok(R::default())
                    },
                },
            }
        })
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

    pub(crate) fn ui(&self) -> impl DerefMut<Target = Ui> + '_ {
        RefMut::map(self.state.borrow_mut(), |state| &mut state.ui)
    }
}

impl Client {
    // Initialization

    /// TODO: docs
    fn setup(&self, preferences: Object) -> crate::Result<()> {
        let state = &mut *self.state.borrow_mut();

        if state.did_setup {
            return Err(Error::AlreadySetup);
        }

        // Set the highlight groups *before* deserializing the preferences so
        // that error messages will be displayed with the right colors.
        hlgroups::setup()?;

        let config = match preferences.kind() {
            ObjectKind::Nil => Config::default(),

            _ => {
                let deserializer = object::Deserializer::new(preferences);
                serde_path_to_error::deserialize::<_, Config>(deserializer)?
            },
        };

        // Only keep the enabled sources
        state
            .sources
            .retain(|&name, _| matches!(config.sources.get(name), Some(true)));

        // Register the main callback on the Neovim thread and spawn the thread
        // pool.
        let (main_sender, pool_sender) = {
            let (ms, handle) = self.register_main_callback()?;
            let sources = state.sources.values().map(Arc::clone).collect();
            let ps = self.start_sources_pool(sources, ms.clone(), handle);
            (ms, ps)
        };

        // Create a new augroup and setup the needed autocommands.
        let augroup_id = {
            let opts = CreateAugroupOpts::builder().clear(true).build();
            let id = api::create_augroup("completion", Some(&opts))?;
            crate::autocmds::setup(self, id)?;
            id
        };

        // Expose a few commands to the users.
        commands::setup(self)?;

        // Finally, update the state.
        state.augroup_id.set(augroup_id);
        state.main_sender.set(main_sender);
        state.pool_sender.set(pool_sender);
        state.config = config;
        state.did_setup = true;

        Ok(())
    }

    /// TODO: docs
    fn register_main_callback(
        &self,
    ) -> nvim::Result<(MainSender, AsyncHandle)> {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        let client = self.clone();

        let handle = nvim::r#loop::new_async(move || {
            match pipeline::main_cb(&client, &mut receiver) {
                Err(Error::NvimError(err)) => return Err(err),
                Err(other) => echoerr!("{:?}", other),
                Ok(_) => {},
            }

            Ok(())
        })?;

        Ok((sender, handle))
    }

    /// TODO: docs
    fn start_sources_pool(
        &self,
        sources: Vec<Arc<dyn CompletionSource>>,
        main_sender: MainSender,
        handle: AsyncHandle,
    ) -> PoolSender {
        let (sender, receiver) = mpsc::unbounded_channel();

        let _ = thread::spawn(move || {
            pipeline::sources_pool(sources, receiver, main_sender, handle)
        });

        sender
    }
}
