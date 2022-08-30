//! TODO: docs

mod completion_config;
mod completion_context;
mod completion_item;
mod completion_request;
// mod completion_source;
mod completion_state;
mod fuzzy;

pub(crate) use completion_config::CompletionConfig;
pub use completion_context::CompletionContext;
pub(crate) use completion_context::Cursor;
pub use completion_item::{CompletionItem, CompletionItemBuilder};
pub(crate) use completion_request::{
    CompletionBundle,
    CompletionRequest,
    RevId,
};
// pub use completion_source::CompletionSource;
// pub(crate) use completion_source::SourceId;
pub(crate) use completion_state::CompletionState;
