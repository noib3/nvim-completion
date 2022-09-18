use completion_types::{
    ClientMessage,
    ClientReceiver,
    ClientSender,
    CoreSender,
    SourceBundle,
};
use tokio::sync::mpsc;

use crate::Core;

/// Starts the completion core on a new thread, returning a [`CoreSender`]
/// which can be used to send messages to the core.
pub fn start(
    sources: Vec<SourceBundle>,
    client_sender: CoreSender,
) -> ClientSender {
    let (core_sender, core_receiver) = mpsc::unbounded_channel();

    let _handle = std::thread::spawn(move || {
        let core = Core::new(sources, client_sender);
        r#loop(core, core_receiver)
    });

    core_sender
}

#[tokio::main]
async fn r#loop(core: Core, mut receiver: ClientReceiver) {
    while let Some(msg) = receiver.recv().await {
        match msg {
            ClientMessage::QueryAttach { document } => {
                core.query_attach(document).await
            },

            ClientMessage::RecomputeCompletions {
                document,
                position,
                revision,
                clock,
            } => {
                core.recompute_completions(document, position, revision, clock)
            },

            // ClientMessage::SendCompletions { revision, from, to } => {
            //     core.send_completions(revision, from, to)
            // },
            ClientMessage::StopSending { revision } => {
                core.stop_sending(revision)
            },
        }
    }
}
