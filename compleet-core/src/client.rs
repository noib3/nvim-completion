use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nvim_oxi::{
    api::Buffer,
    Dictionary,
    FromObject,
    Function,
    Object,
    ToObject,
};

use crate::{messages, setup};
use crate::{CompletionSource, Config, Error};

#[derive(Default)]
pub struct Client(Rc<RefCell<State>>);

#[derive(Default)]
pub(crate) struct State {
    /// The id of the `Compleet` augroup if currently set, `None` otherwise.
    augroup_id: Option<u32>,

    /// Whether the [`setup`](setup::setup) function has ever been called.
    did_setup: bool,

    sources: Vec<Arc<dyn CompletionSource>>,
}

impl From<&Rc<RefCell<State>>> for Client {
    fn from(state: &Rc<RefCell<State>>) -> Self {
        Self(Rc::clone(&state))
    }
}

impl Client {
    #[inline]
    pub(crate) fn already_setup(&self) -> bool {
        self.0.borrow().did_setup
    }

    /// Returns a [`Dictionary`] representing the public API of the plugin.
    pub fn build_api(&self) -> Dictionary {
        Dictionary::from_iter([("setup", Object::from(self.setup()))])
    }

    pub(crate) fn create_fn<F, A, R, E>(&self, fun: F) -> Function<A, R>
    where
        F: Fn(&Self, A) -> Result<R, E> + 'static,
        A: FromObject,
        R: ToObject + Default,
        E: Into<Error>,
    {
        let state = Rc::clone(&self.0);
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
        self.0.borrow_mut().did_setup = true;
    }

    #[inline]
    /// Creates a new [`Client`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_source<S>(&self, source: S)
    where
        S: CompletionSource,
    {
        let sources = &mut self.0.borrow_mut().sources;
        sources.push(Arc::new(source));
    }

    pub(crate) fn setup(&self) -> Function<Object, ()> {
        self.create_fn(setup::setup)
    }

    pub(crate) fn set_config(&self, config: Config) {
        // todo!()
    }

    /// Queries all the registered completion sources, returning whether any of
    /// them are enabled for the given `buf`.
    pub(crate) fn should_attach(&self, buf: &Buffer) -> bool {
        todo!()
    }
}
