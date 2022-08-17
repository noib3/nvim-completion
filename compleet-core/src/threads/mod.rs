mod setup;
mod thread_main;
mod thread_pool;

pub(crate) use setup::setup;
use thread_main::main_cb;
pub(crate) use thread_main::MainMessage;
use thread_pool::sources_pool;
pub(crate) use thread_pool::PoolMessage;
