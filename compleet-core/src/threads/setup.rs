use std::sync::Arc;
use std::thread;

use nvim_oxi::r#loop;
use tokio::sync::mpsc::{self, UnboundedReceiver};

use super::{MainMessage, PoolMessage};
use crate::client::Client;
use crate::CompletionSource;

pub(crate) fn setup(
    client: Client,
    sources: Vec<Arc<dyn CompletionSource>>,
    pool_receiver: UnboundedReceiver<PoolMessage>,
) -> crate::Result<()> {
    let (main_sender, mut main_receiver) =
        mpsc::unbounded_channel::<MainMessage>();

    let handle = r#loop::new_async(move || {
        super::main_cb(&client, &mut main_receiver).map_err(|err| match err {
            crate::Error::NvimError(nvim_err) => nvim_err,
            _ => todo!(),
        })
    })?;

    let _ = thread::spawn(move || {
        super::sources_pool(sources, pool_receiver, main_sender, handle)
    });

    Ok(())
}
