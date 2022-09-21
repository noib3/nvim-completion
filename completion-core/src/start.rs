use std::{panic, thread};

use completion_types::{
    ClientMessage,
    ClientReceiver,
    CoreMessage,
    CoreSender,
    SourceBundle,
};

use crate::{Result, State};

/// Starts the completion core on a new thread.
pub fn start(
    sources: Vec<SourceBundle>,
    core_sender: CoreSender,
    client_receiver: ClientReceiver,
) {
    let sender = core_sender.clone();

    panic::set_hook(Box::new(move |infos| {
        let thread_name =
            thread::current().name().unwrap_or("<unnamed>").to_owned();

        let message = match infos.payload().downcast_ref::<&'static str>() {
            Some(s) => Some((*s).to_owned()),

            None => match infos.payload().downcast_ref::<String>() {
                Some(s) => Some(s.to_owned()),
                None => None,
            },
        };

        let location = infos.location().map(|location| {
            (location.line(), location.column(), location.file().to_owned())
        });

        sender.send(CoreMessage::CorePanicked {
            thread_name,
            message,
            location,
        });
    }));

    thread::spawn(move || {
        let core = State::new(sources, core_sender.clone());

        match self::event_loop(core, client_receiver) {
            Err(error) => {
                core_sender.send(CoreMessage::CoreFailed(Box::new(error) as _))
            },

            Ok(()) => unreachable!(
                "we only exit the event loop if an error is returned"
            ),
        }
    });
}

#[tokio::main]
async fn event_loop(core: State, mut receiver: ClientReceiver) -> Result<()> {
    while let Some(msg) = receiver.recv().await {
        match msg {
            ClientMessage::QueryAttach { document } => {
                core.query_attach(document)?
            },

            ClientMessage::CompletionRequest { request } => {
                core.recompute_completions(request)?
            },

            ClientMessage::CancelRequest { revision } => {
                core.stop_sending(revision)?
            },
        }
    }

    Ok(())
}
