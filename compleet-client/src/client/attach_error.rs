use bindings::opinionated::buffer::Buffer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AttachError {
    #[error("{0} is already attached")]
    AlreadyAttached(Buffer),

    #[error("{0} is not modifiable")]
    NotModifiable(Buffer),

    #[error("there are no completion sources enabled for {0}")]
    NoSourcesEnabled(Buffer),

    #[error("attach failed")]
    AttachFailed,

    #[error(transparent)]
    Other(#[from] mlua::Error),
}
