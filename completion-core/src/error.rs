use std::error::Error as StdError;

use nvim_oxi as nvim;
use thiserror::Error as ThisError;

use crate::sources::SourceId;

pub(crate) type GenericError = Box<dyn StdError + Send + Sync + 'static>;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[doc(hidden)]
#[derive(Debug, ThisError)]
#[non_exhaustive]
pub enum Error {
    #[error("can't setup more than once per session")]
    AlreadySetup,

    #[error("error parsing `{prefix}{option}`: {why}")]
    BadConfig {
        prefix: String,
        option: serde_path_to_error::Path,
        why: String,
    },

    // For some reason I can't use `source` as a name field??
    #[error("error trying to attach source `{sauce}`: {why}")]
    SourceAttach { sauce: SourceId, why: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Loop(#[from] nvim_oxi::r#loop::Error),

    #[error(transparent)]
    Nvim(#[from] nvim_oxi::Error),

    #[error(transparent)]
    OneshotRecv(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("{0}")]
    Other(String),
}

impl From<serde_path_to_error::Error<nvim::Error>> for Error {
    fn from(err: serde_path_to_error::Error<nvim::Error>) -> Self {
        let option = err.path().to_owned();

        match err.into_inner() {
            nvim::Error::DeserializeError(why) => {
                Self::BadConfig { prefix: "".into(), option, why }
            },

            other => other.into(),
        }
    }
}

impl Error {
    /// TODO: docs
    pub(crate) fn source_attach<E: StdError>(
        source: SourceId,
        err: E,
    ) -> Self {
        Self::SourceAttach { sauce: source, why: format!("{}", err) }
    }

    /// TODO: docs
    pub(crate) fn source_deser(
        err: serde_path_to_error::Error<nvim::Error>,
        name: &'static str,
    ) -> Self {
        let option = err.path().to_owned();

        match err.into_inner() {
            nvim::Error::DeserializeError(why) => Self::BadConfig {
                prefix: format!("sources.{}.", name),
                option,
                why,
            },

            other => other.into(),
        }
    }
}
