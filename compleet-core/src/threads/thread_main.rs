use std::sync::Arc;
use std::time::Instant;

use nvim_oxi as nvim;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{Client, CompletionItem};

/// Messages sent from the thread pool to the main thread.
#[derive(Debug)]
pub(crate) enum MainMessage {
    /// TODO: docs
    AttachBuf(Arc<crate::Buffer>),

    /// TODO: docs
    ShowCompletions(crate::Result<Vec<CompletionItem>>, Arc<Instant>),
}

pub(super) fn main_cb(
    client: &Client,
    receiver: &mut UnboundedReceiver<MainMessage>,
) -> crate::Result<()> {
    let mut completions = Vec::new();

    while let Ok(msg) = receiver.try_recv() {
        match msg {
            MainMessage::ShowCompletions(Ok(cmp), start) => {
                completions.push((cmp.into_iter().next().unwrap(), start))
            },

            MainMessage::ShowCompletions(Err(_err), _) => todo!(),

            MainMessage::AttachBuf(buf) => client.attach_buffer(buf)?,
        }
    }

    if !completions.is_empty() {
        self::show_completions(&client, completions);
    }

    Ok(())
}

/// TODO: docs
fn show_completions(
    _client: &Client,
    completions: Vec<(CompletionItem, Arc<Instant>)>,
) {
    nvim::schedule(move |_| {
        for (cmp, start) in completions {
            nvim::print!("{}, {:?}", cmp.text, start.elapsed());
        }

        Ok(())
    })
}
