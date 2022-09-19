use std::sync::{MutexGuard, PoisonError};

use thiserror::Error as ThisError;

use crate::State;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("a thread panicked while holding the lock: {0}")]
    StateMutexPoisoned(String),
}

// Only keeping the error message to not have to deal w/ lifetimes.
impl From<PoisonError<MutexGuard<'_, State>>> for Error {
    fn from(err: PoisonError<MutexGuard<'_, State>>) -> Self {
        Self::StateMutexPoisoned(err.to_string())
    }
}
