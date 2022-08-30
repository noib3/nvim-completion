use nvim_oxi as nvim;
use thiserror::Error as ThisError;

/// `nvim-completion`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
#[non_exhaustive]
pub enum Error {
    #[error("can't setup more than once per session")]
    AlreadySetup,

    #[error("error parsing `{option}`: {why}")]
    BadPreferences { option: serde_path_to_error::Path, why: String },

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
                Self::BadPreferences { option, why }
            },

            other => other.into(),
        }
    }
}
