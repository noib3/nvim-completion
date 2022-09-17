//! TODO: docs

mod accept_completion;
mod scroll_details;
mod select_completion;
mod setup;
mod show_completions;

use accept_completion::accept_completion;
use scroll_details::scroll_details;
use select_completion::select_completion;
pub(crate) use setup::setup;
use setup::IdentifyCompletion;
use show_completions::show_completions;
