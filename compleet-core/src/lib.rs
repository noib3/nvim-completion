mod autocmds;
mod client;
mod commands;
mod completion_source;
mod config;
mod edit;
mod error;
mod hlgroups;
mod mappings;
mod messages;
mod on_bytes;
mod setup;

pub use client::Client;
pub use completion_source::CompletionSource;
pub use error::{Error, Result};
