use std::ops::{Deref, DerefMut};

use once_cell::unsync::{self, OnceCell};

/// A thin object wrapper used to mimick late initialization ala Kotlin.
#[derive(Debug, Clone)]
pub(crate) struct LateInit<T>(unsync::OnceCell<T>);

impl<T> LateInit<T> {
    #[inline]
    pub fn set(&mut self, value: T) {
        if let Err(_) = self.0.set(value) {
            panic!("couldn't set LateInit");
        }
    }
}

impl<T> Default for LateInit<T> {
    #[inline]
    fn default() -> Self {
        Self(OnceCell::default())
    }
}

impl<T> Deref for LateInit<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.get().unwrap()
    }
}

impl<T> DerefMut for LateInit<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
}
