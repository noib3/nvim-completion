use std::error::Error as StdError;
use std::sync::Arc;

use nvim::{api::Buffer as NvimBuffer, Function};
use nvim_oxi as nvim;
use tokio::sync::{mpsc, oneshot};

use crate::completions::CompletionBundle;
use crate::messages::echoerr;
use crate::sources::SourceId;
use crate::{Client, Result};

type MainReceiver = mpsc::UnboundedReceiver<MainMessage>;

/// TODO: docs
#[derive(Clone)]
pub(crate) struct MainSender {
    sender: mpsc::UnboundedSender<MainMessage>,
    handle: nvim::r#loop::AsyncHandle,
}

impl MainSender {
    pub(crate) fn new(
        sender: mpsc::UnboundedSender<MainMessage>,
        handle: nvim::r#loop::AsyncHandle,
    ) -> Self {
        Self { sender, handle }
    }

    pub(crate) fn send(&self, msg: MainMessage) {
        self.sender.send(msg).unwrap();
        self.handle.send().unwrap();
    }
}

/// Messages sent from the thread pool to the main thread.
#[derive(Debug)]
pub(crate) enum MainMessage {
    /// TODO: docs
    AttachBuf(Arc<crate::Buffer>),

    /// TODO: docs
    HandleCompletions(CompletionBundle),

    /// TODO: docs
    QueryAttach(Function<NvimBuffer, bool>, NvimBuffer, oneshot::Sender<bool>),

    /// A completion source returned an error while executing its
    /// [`complete`](crate::CompletionSource::complete) implementation.
    CompleteFailed {
        on_source: SourceId,
        with_error: Box<dyn StdError + Send + Sync + 'static>,
    },
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

            CompleteFailed { on_source, with_error } => {
                nvim::schedule(move |_| {
                    echoerr!(
                        "source `{}` failed to compute completions: {}",
                        on_source,
                        with_error
                    );

                    Ok(())
                })
            },
        }
    }

    if !bundles.is_empty() {
        super::on_completions_arrival(client, bundles)?;
    }

    Ok(())
}
