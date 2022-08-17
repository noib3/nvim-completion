use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use nvim_oxi::{
    api::Buffer,
    Dictionary,
    Function,
    LuaPoppable,
    LuaPushable,
    Object,
};
use tokio::sync::mpsc;

use crate::channels::{self, PoolMessage};
use crate::config::{Config, SOURCE_NAMES};
use crate::{mappings, messages, setup};
use crate::{CompletionContext, CompletionSource, Error};

#[derive(Default)]
pub struct Client {
    state: Rc<RefCell<State>>,
}

#[derive(Default)]
struct State {
    /// The id of the `Compleet` augroup if currently set, `None` otherwise.
    augroup_id: Option<u32>,

    /// Every attached buffer has its own completion context...
    contexts: HashMap<Buffer, Arc<CompletionContext>>,

    /// TODO: docs
    pool_sender: Option<mpsc::UnboundedSender<PoolMessage>>,

    /// Whether the [`setup`](setup::setup) function has ever been called.
    did_setup: bool,

    sources: HashMap<&'static str, Arc<dyn CompletionSource>>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self { state: Rc::clone(&self.state) }
    }
}

impl From<&Rc<RefCell<State>>> for Client {
    fn from(state: &Rc<RefCell<State>>) -> Self {
        Self { state: Rc::clone(&state) }
    }
}

impl Client {
    /// Creates a new [`Client`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn already_setup(&self) -> bool {
        self.state.borrow().did_setup
    }

    // /// TODO: docs
    // pub(crate) fn apply_edit<'ins>(
    //     &self,
    //     buf: &Buffer,
    //     edit: Edit<'ins>,
    // ) -> crate::Result<()> {
    //     let state = &mut self.state.borrow_mut();
    //     let rope = state.buffers.get_mut(buf).ok_or(Error::AlreadySetup)?;
    //     edit.apply_to_rope(rope);
    //     Ok(())
    // }

    pub(crate) fn add_context(&self, buf: Buffer, ctx: CompletionContext) {
        let state = &mut *self.state.borrow_mut();
        state.contexts.insert(buf, Arc::new(ctx));
    }

    /// Returns a [`Dictionary`] representing the public API of the plugin.
    pub fn build_api(&self) -> Dictionary {
        [("setup", Object::from(self.create_fn(setup::setup)))]
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

    pub(crate) fn create_fn<F, A, R, E>(&self, fun: F) -> Function<A, R>
    where
        F: Fn(&Self, A) -> Result<R, E> + 'static,
        A: LuaPoppable,
        R: LuaPushable + Default,
        E: Into<Error>,
    {
        let client = self.clone();
        Function::from_fn(move |args| {
            match fun(&client, args).map_err(Into::into) {
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

    /// TODO: docs
    pub(crate) fn get_context(&self, buf: &Buffer) -> Arc<CompletionContext> {
        let state = self.state.borrow();
        Arc::clone(state.contexts.get(buf).unwrap())
    }

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

    pub(crate) fn set_config(&self, config: Config) {
        let state = &mut *self.state.borrow_mut();
        state.sources.retain(|name, _| {
            config.sources.get(*name).map(|enable| *enable).unwrap_or_default()
        });
    }

    pub(crate) fn start_channel(&self) -> crate::Result<()> {
        let state = &mut *self.state.borrow_mut();

        let (sender, recv) = mpsc::unbounded_channel();

        state.pool_sender = Some(sender);

        let sources =
            state.sources.values().map(Arc::clone).collect::<Vec<_>>();

        channels::setup(self, sources, recv)
    }

    // -----------------------------------------------------------------------
    // Thread pool messaging.

    /// Sends a message to the thread pool.
    #[inline]
    fn send_pool_msg(&self, msg: PoolMessage) {
        self.state.borrow().pool_sender.as_ref().unwrap().send(msg).unwrap();
    }

    // TODO: docs
    #[inline]
    pub(crate) fn query_attach(&self, buf: Buffer) {
        self.send_pool_msg(PoolMessage::QueryAttach(buf))
    }

    // TODO: docs
    #[inline]
    pub(crate) fn query_completions(&self, ctx: Arc<CompletionContext>) {
        self.send_pool_msg(PoolMessage::QueryCompletions(ctx))
    }

    // TODO: docs
    #[inline]
    pub(crate) fn stop_sources(&self) {
        self.send_pool_msg(PoolMessage::AbortAll);
    }
}
