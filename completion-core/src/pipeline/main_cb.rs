use std::sync::Arc;

use nvim_oxi::{api::Buffer as NvimBuffer, Function};
use tokio::sync::{mpsc, oneshot};

use crate::completions::CompletionBundle;
use crate::{Client, Result};

pub(crate) type MainSender = mpsc::UnboundedSender<MainMessage>;
type MainReceiver = mpsc::UnboundedReceiver<MainMessage>;

/// Messages sent from the thread pool to the main thread.
#[derive(Debug)]
pub(crate) enum MainMessage {
    /// TODO: docs
    AttachBuf(Arc<crate::Buffer>),

    /// TODO: docs
    HandleCompletions(CompletionBundle),

    /// TODO: docs
    QueryAttach(Function<NvimBuffer, bool>, NvimBuffer, oneshot::Sender<bool>),
}

/// TODO: docs
pub(crate) fn main_cb(
    client: &Client,
    receiver: &mut MainReceiver,
) -> Result<()> {
    let mut bundles = Vec::<CompletionBundle>::new();

    use MainMessage::*;

    while let Ok(msg) = receiver.try_recv() {
        match msg {
            AttachBuf(buf) => client.attach_buffer(buf)?,

            HandleCompletions(bundle) => bundles.push(bundle),

            QueryAttach(fun, buf, sender) => {
                let res = fun
                    .call(buf)
                    .map_err(|err| crate::Error::source_attach("??", err))?;

                sender.send(res).unwrap()
            },
        }
    }

    if !bundles.is_empty() {
        super::on_completions_arrival(client, bundles)?;
    }

    Ok(())
}
