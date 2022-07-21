use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nvim_oxi::{Function, Object};

use super::setup;
use crate::messages;
use crate::{CompletionSource, Error};

#[derive(Default)]
pub struct Client(Rc<RefCell<State>>);

#[derive(Default)]
pub(crate) struct State {
    /// Whether the [`setup`](setup::setup) function has ever been called.
    did_setup: bool,

    sources: Vec<Arc<dyn CompletionSource>>,
}

impl Client {
    #[inline]
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

    pub fn setup(&self) -> Function<Object, ()> {
        let state = Rc::clone(&self.0);

        Function::from_fn(move |preferences| {
            if let Err(err) =
                setup::setup(&mut state.borrow_mut(), preferences)
            {
                use Error::*;
                if matches!(err, AlreadySetup | BadPreferences { .. }) {
                    // messages::echoerr("{err}");
                }
            }

            Ok(())
        })
    }
}

impl State {
    #[inline]
    pub const fn already_setup(&self) -> bool {
        self.did_setup
    }
}
