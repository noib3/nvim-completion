mod autocmds;
mod buffer;
mod client;
mod commands;
mod completions;
mod config;
mod error;
mod hlgroups;
mod lateinit;
mod mappings;
mod messages;
mod pipeline;
mod setup;
mod sources;
mod ui;
mod utils;

pub use buffer::Buffer;
pub use client::Client;
pub use completions::{
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
};
use error::{Error, GenericError, Result};
pub use setup::{build_api, register_source};
pub use sources::CompletionSource;
