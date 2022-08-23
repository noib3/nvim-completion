use std::sync::Arc;
use std::thread;

use nvim_oxi::r#loop;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::{MainMessage, PoolMessage};
use crate::client::Client;
use crate::CompletionSource;

pub(crate) fn setup(
    client: Client,
    sources: Vec<Arc<dyn CompletionSource>>,
    cb_sender: UnboundedSender<MainMessage>,
    mut cb_receiver: UnboundedReceiver<MainMessage>,
    pool_receiver: UnboundedReceiver<PoolMessage>,
) -> crate::Result<()> {
    let handle = r#loop::new_async(move || {
        super::main_cb(&client, &mut cb_receiver).map_err(|err| match err {
            crate::Error::NvimError(nvim_err) => nvim_err,
            _ => todo!(),
        })
    })?;

    let _ = thread::spawn(move || {
        super::sources_pool(sources, pool_receiver, cb_sender, handle)
    });

    Ok(())
}
