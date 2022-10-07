use completion_types::{ClientMessage, GenericError, SourceId};
use nvim_oxi as nvim;
use thiserror::Error as ThisError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub(crate) enum Error {
    #[error("can't setup more than once per session")]
    AlreadySetup,

    #[error("error parsing `{prefix}{option}`: {why}")]
    BadConfig {
        prefix: String,
        option: serde_path_to_error::Path,
        why: String,
    },

    #[error("core returned with error: {0}")]
    CoreFailed(GenericError),

    #[error("{0}")]
    CorePanicked(String),

    // For some reason I can't use `source` as a name field??
    #[error("error trying to attach source `{sauce}`: {why}")]
    SourceCompleteFailed { sauce: SourceId, why: String },

    #[error("source `{sauce}` failed to compute completions: {why}")]
    SourceEnableFailed { sauce: SourceId, why: String },

    #[error(transparent)]
    NvimLoop(#[from] nvim_oxi::libuv::Error),

    #[error(transparent)]
    Nvim(#[from] nvim_oxi::Error),

    #[error(transparent)]
    NvimApi(#[from] nvim_oxi::api::Error),

    #[error(transparent)]
    NvimSerde(#[from] nvim_oxi::serde::Error),

    #[error(transparent)]
    OneshotRecv(#[from] tokio::sync::oneshot::error::RecvError),

    #[error(transparent)]
    ClientSendError(
        #[from] tokio::sync::mpsc::error::SendError<ClientMessage>,
    ),
}

impl From<serde_path_to_error::Error<nvim::serde::Error>> for Error {
    fn from(err: serde_path_to_error::Error<nvim::serde::Error>) -> Self {
        Self::BadConfig {
            prefix: "".into(),
            option: err.path().to_owned(),
            why: err.into_inner().to_string(),
        }
    }
}

impl Error {
    #[inline]
    pub(crate) fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::CoreFailed(_)
                | Self::CorePanicked(_)
                | Self::NvimLoop(_)
                | Self::ClientSendError(_)
        )
    }

    pub(crate) fn core_panicked(
        thread_name: &str,
        message: Option<&str>,
        location: Option<(u32, u32, String)>,
    ) -> Self {
        let message = message.map(|msg| format!(" at '{}'", msg));

        let location = location
            .map(|(line, _, filename)| format!(": {}:L{}", filename, line));

        let msg = format!(
            "core panicked on thread '{}'{}{}",
            thread_name,
            message.unwrap_or_default(),
            location.unwrap_or_default()
        );

        Self::CorePanicked(msg)
    }
}
