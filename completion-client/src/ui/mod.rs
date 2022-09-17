//! TODO: docs

mod completion_details;
mod completion_hint;
mod completion_menu;
mod config;
mod geometry;
mod utils;

pub(crate) use completion_details::CompletionItemDetails;
use completion_details::DetailsConfig;
pub(crate) use completion_hint::CompletionHint;
use completion_hint::HintConfig;
pub(crate) use completion_menu::CompletionMenu;
use completion_menu::MenuConfig;
use config::Border;
pub(crate) use config::UiConfig;
use geometry::*;
