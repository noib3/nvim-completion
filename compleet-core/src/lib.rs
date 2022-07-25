mod autocmds;
mod client;
mod commands;
mod completion_source;
mod config;
mod error;
mod hlgroups;
mod mappings;
mod messages;
mod setup;

pub use client::Client;
pub use completion_source::CompletionSource;
pub use error::{Error, Result};
