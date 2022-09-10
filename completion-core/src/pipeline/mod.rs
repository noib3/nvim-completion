//! TODO: docs

mod fuzzy;
mod main_cb;
mod on_bytes;
mod on_completions_arrival;
mod sources_pool;

use fuzzy::fuzzy_find;
pub(crate) use main_cb::{main_cb, MainMessage, MainSender};
pub(crate) use on_bytes::on_bytes;
pub(crate) use on_completions_arrival::on_completions_arrival;
pub(crate) use sources_pool::{sources_pool, PoolMessage, PoolSender};
