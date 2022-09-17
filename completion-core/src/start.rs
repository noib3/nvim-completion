use completion_types::{
    ClientSender,
    CoreMessage,
    CoreReceiver,
    CoreSender,
    SourceBundle,
};
use tokio::sync::mpsc;

use crate::State;

/// Starts the completion core on a new thread, returning a [`CoreSender`]
/// which can be used to send messages to the core.
pub fn start(
    sources: Vec<SourceBundle>,
    client_sender: ClientSender,
) -> CoreSender {
    let (core_sender, core_receiver) = mpsc::unbounded_channel();

    let _handle = std::thread::spawn(move || {
        let mut state = State::new(sources, client_sender);
        r#loop(&mut state, core_receiver)
    });

    core_sender
}

#[tokio::main]
async fn r#loop(state: &mut State, mut receiver: CoreReceiver) {
    while let Some(msg) = receiver.recv().await {
        match msg {
            CoreMessage::QueryAttach { document } => {
                state.query_attach(document).await
            },

            CoreMessage::RecomputeCompletions {
                document,
                position,
                revision,
                clock,
            } => {
                state
                    .recompute_completions(document, position, revision, clock)
                    .await
            },

            CoreMessage::SendCompletions { revision, from, to } => {
                state.send_completions(revision, from, to)
            },

            CoreMessage::StopSending { revision } => {
                state.stop_sending(revision)
            },
        }
    }
}
