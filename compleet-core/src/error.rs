use nvim_oxi as nvim;

/// Alias for a `Result` with error type [`nvim_compleet::Error`](Error).
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("can't setup more than once per session")]
    AlreadySetup,

    #[error("error parsing `{option}`: {why}")]
    BadPreferences { option: serde_path_to_error::Path, why: String },

    #[error(transparent)]
    NvimError(#[from] nvim_oxi::Error),

    #[error(transparent)]
    LoopError(#[from] nvim_oxi::r#loop::Error),
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
