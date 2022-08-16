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
use ropey::{Rope, RopeBuilder};
use tokio::sync::mpsc;

use crate::config::{Config, SOURCE_NAMES};
use crate::edit::Edit;
use crate::{channels, mappings, messages, setup};
use crate::{CompletionContext, CompletionSource, Error};

#[derive(Default)]
pub struct Client {
    state: Rc<RefCell<State>>,
}

#[derive(Default)]
struct State {
    /// The id of the `Compleet` augroup if currently set, `None` otherwise.
    augroup_id: Option<u32>,

    /// Mapping from [`Buffer`]s to buffer contents represented as [`Rope`]s.
    buffers: HashMap<Buffer, Rope>,

    /// TODO: docs
    ctx_sender: Option<mpsc::UnboundedSender<Arc<crate::CompletionContext>>>,

    /// Whether the [`setup`](setup::setup) function has ever been called.
    did_setup: bool,

    sources: HashMap<&'static str, Arc<dyn CompletionSource>>,
}

impl From<&Rc<RefCell<State>>> for Client {
    fn from(state: &Rc<RefCell<State>>) -> Self {
        Self { state: Rc::clone(&state) }
    }
}

impl Client {
    #[inline]
    pub(crate) fn already_setup(&self) -> bool {
        self.state.borrow().did_setup
    }

    /// TODO: docs
    pub(crate) fn apply_edit<'ins>(
        &self,
        buf: &Buffer,
        edit: Edit<'ins>,
    ) -> crate::Result<()> {
        let state = &mut self.state.borrow_mut();
        let rope = state.buffers.get_mut(buf).ok_or(Error::AlreadySetup)?;
        edit.apply_to_rope(rope);
        Ok(())
    }

    /// Attaches a new buffer by ...
    pub(crate) fn attach_buffer(&self, buf: Buffer) -> crate::Result<()> {
        let mut builder = RopeBuilder::new();
        for line in buf.get_lines(0, buf.line_count()?, true)? {
            builder.append(&line.to_string_lossy());
        }

        let state = &mut self.state.borrow_mut();
        state.buffers.insert(buf, builder.finish());

        Ok(())
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
        let state = Rc::clone(&self.state);
        Function::from_fn(move |args| {
            match fun(&Client::from(&state), args).map_err(Into::into) {
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

    /// Creates a new [`Client`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
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

    pub(crate) fn send_ctx(&self, ctx: CompletionContext) {
        let _ = self
            .state
            .borrow()
            .ctx_sender
            .as_ref()
            .unwrap()
            .send(Arc::new(ctx));
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

        state.ctx_sender = Some(sender);

        let sources =
            state.sources.values().map(Arc::clone).collect::<Vec<_>>();

        channels::setup(sources, recv)
    }

    /// Queries all the registered completion sources, returning whether any of
    /// them are enabled for the given `buf`.
    pub(crate) fn should_attach(&self, buf: &Buffer) -> crate::Result<bool> {
        buf.get_option("modifiable").map_err(Into::into)
    }
}
