mod client_message;
mod clock;
mod completion_item;
mod completion_list;
mod completion_source;
mod core_message;
mod document;
mod position;
mod revision;
mod scored_completion;
mod source_bundle;
mod source_enable;

pub use client_message::{ClientMessage, ClientReceiver, ClientSender};
pub use clock::Clock;
pub use completion_item::{CompletionItem, CompletionItemBuilder};
pub use completion_list::CompletionList;
pub use completion_source::{CompletionSource, ObjectSafeCompletionSource};
pub use core_message::{CoreMessage, CoreReceiver, CoreSender};
pub use document::Document;
pub use position::Position;
pub use revision::Revision;
pub use scored_completion::ScoredCompletion;
pub use source_bundle::{SourceBundle, SourceId};
pub use source_enable::SourceEnable;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

// // #[cfg(any(feature = "ui", feature = "core"))]
// mod internal {}

// // #[cfg(any(feature = "ui", feature = "core"))]
// pub use internal::*;
