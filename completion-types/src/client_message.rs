use std::sync::Arc;

use nvim_oxi as nvim;
use nvim_oxi::r#loop::AsyncHandle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

use crate::{
    Clock,
    CompletionItem,
    Document,
    GenericError,
    Revision,
    SourceId,
};

pub type ClientReceiver = UnboundedReceiver<ClientMessage>;

/// Message sender used by the core to send messages to the UI thread managed
/// by Neovim.
#[derive(Clone)]
pub struct ClientSender {
    sender: UnboundedSender<ClientMessage>,
    trigger: AsyncHandle,
}

// TODO: why?
unsafe impl Sync for ClientSender {}

// Custom `Debug` impl because `AsyncHandle` doesn't implement `Debug`.
impl std::fmt::Debug for ClientSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiSender").field("sender", &self.sender).finish()
    }
}

impl ClientSender {
    #[inline(always)]
    pub fn new(
        sender: UnboundedSender<ClientMessage>,
        trigger: AsyncHandle,
    ) -> Self {
        Self { sender, trigger }
    }

    #[inline(always)]
    pub fn send(&self, msg: ClientMessage) {
        let _ = self.sender.send(msg);
        self.trigger.send().unwrap();
    }

    pub async fn on_nvim_thread<F, T>(&self, fun: F) -> T
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();

        let fun = Box::new(move |()| {
            let _ = sender.send(fun());
            Ok::<_, nvim::Error>(())
        });

        self.send(ClientMessage::ExecuteLuaFunction { fun });

        receiver.await.unwrap()
    }
}

/// Messages sent from the core to the UI thread.
pub enum ClientMessage {
    /// TODO: docs
    AttachDocument { document: Arc<Document> },

    /// TODO: docs
    ExecuteLuaFunction {
        fun: Box<dyn FnOnce(()) -> Result<(), nvim::Error> + Send>,
    },

    /// A completion source returned an error while executing its
    /// [`enable`](crate::CompletionSource::enable) implementation.
    SourceEnableFailed { source: SourceId, error: GenericError },

    /// A completion source returned an error while executing its
    /// [`complete`](crate::CompletionSource::complete) implementation.
    SourceCompleteFailed { source: SourceId, error: GenericError },

    /// TODO: docs
    Completions {
        items: Vec<Arc<CompletionItem>>,
        from: u32,
        to: u32,
        revision: Revision,
        clock: Clock,
    },
}
