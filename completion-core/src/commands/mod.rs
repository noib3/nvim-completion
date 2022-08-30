//! TODO: docs

mod completion_start;
mod completion_stats;
mod completion_stop;
mod setup;

use completion_start::completion_start;
use completion_stats::completion_stats;
use completion_stop::completion_stop;
pub(crate) use setup::setup;
