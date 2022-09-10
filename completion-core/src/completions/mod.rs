//! TODO: docs

mod completion_config;
mod completion_context;
mod completion_item;
mod completion_request;
mod completion_state;

pub(crate) use completion_config::CompletionConfig;
pub use completion_context::CompletionContext;
pub(crate) use completion_context::LineContext;
pub use completion_item::{CompletionItem, CompletionItemBuilder};
pub(crate) use completion_request::{
    CompletionBundle,
    CompletionRequest,
    RevId,
};
pub(crate) use completion_state::CompletionState;
