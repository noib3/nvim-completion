use nvim_oxi::{self as nvim, api::Buffer};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{Client, CompletionContext, CompletionItem};

/// Messages sent from the thread pool to the main thread.
#[derive(Debug)]
pub(crate) enum MainMessage {
    /// TODO: docs
    AttachBuf(Buffer),

    /// TODO: docs
    ShowCompletions(crate::Result<Vec<CompletionItem>>),
}

pub(super) fn main_cb(
    client: &Client,
    receiver: &mut UnboundedReceiver<MainMessage>,
) -> crate::Result<()> {
    let mut completions = Vec::new();

    while let Ok(msg) = receiver.try_recv() {
        match msg {
            MainMessage::ShowCompletions(Ok(cmp)) => {
                completions.extend(cmp.into_iter().take(1))
            },

            MainMessage::ShowCompletions(Err(_err)) => todo!(),

            MainMessage::AttachBuf(buf) => {
                self::attach_buffer(&client, buf).unwrap()
            },
        }
    }

    if !completions.is_empty() {
        self::show_completions(&client, completions);
    }

    Ok(())
}

/// TODO: docs
fn attach_buffer(client: &Client, buf: Buffer) -> crate::Result<()> {
    let on_bytes = client.create_fn(crate::on_bytes::on_bytes);

    let opts = nvim::opts::BufAttachOpts::builder().on_bytes(on_bytes).build();
    buf.attach(false, &opts)?;

    let ctx = CompletionContext::new(buf.clone());
    client.add_context(buf, ctx);

    Ok(())
}

///
fn show_completions(_client: &Client, completions: Vec<CompletionItem>) {
    nvim::schedule(move |_| {
        let time = std::time::Instant::now();
        for cmp in completions {
            nvim::print!("{}, {:?}", cmp.text, time);
        }

        Ok(())
    })
}
