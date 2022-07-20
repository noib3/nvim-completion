use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nvim_oxi::{Function, Object};

use super::setup;
use crate::CompletionSource;

#[derive(Default)]
pub struct Client(Rc<RefCell<State>>);

#[derive(Default)]
pub(crate) struct State {
    sources: Vec<Arc<dyn CompletionSource>>,
}

impl Client {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self) -> Function<Object, ()> {
        let state = Rc::clone(&self.0);
        Function::from_fn(move |preferences| {
            setup::setup(&mut state.borrow_mut(), preferences)
        })
    }
}
