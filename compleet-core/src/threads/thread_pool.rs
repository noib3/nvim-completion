use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::{FuturesOrdered, StreamExt};
use nvim_oxi::api::Buffer;
use nvim_oxi::r#loop::AsyncHandle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use super::MainMessage;
use crate::{CompletionContext, CompletionSource};

/// Messages sent from the main thread to the thread pool.
#[derive(Debug)]
pub(crate) enum PoolMessage {
    /// TODO: docs
    AbortAll,

    /// TODO: docs
    QueryAttach(Buffer),

    /// TODO: docs
    QueryCompletions(Arc<CompletionContext>),
}

/// TODO: let this thread pool own the sources which are currently stored as a
/// hashmap on the UI thread?
#[tokio::main]
pub(super) async fn sources_pool(
    sources: Vec<Arc<dyn CompletionSource>>,
    mut receiver: UnboundedReceiver<PoolMessage>,
    sender: UnboundedSender<MainMessage>,
    mut cb_handle: AsyncHandle,
) {
    // TODO: docs
    let mut buf_sources =
        HashMap::<Buffer, Vec<Arc<dyn CompletionSource>>>::new();

    // TODO: docs
    let mut handles = Vec::<JoinHandle<()>>::new();

    while let Some(msg) = receiver.recv().await {
        match msg {
            PoolMessage::AbortAll => {
                handles.drain(..).for_each(|task| task.abort());
            },

            PoolMessage::QueryAttach(buf) => {
                let sources =
                    self::attach_enabled_sources(&sources, &buf).await;

                if !sources.is_empty() {
                    buf_sources.insert(buf.clone(), sources);
                    sender.send(MainMessage::AttachBuf(buf)).unwrap();
                    cb_handle.send().unwrap();
                }
            },

            PoolMessage::QueryCompletions(ctx) => {
                handles.drain(..).for_each(|task| task.abort());

                handles = self::send_completions(
                    buf_sources.get(ctx.buf()).unwrap(),
                    ctx,
                    &sender,
                    &cb_handle,
                )
                .await;
            },
        }
    }
}

/// TODO: docs
async fn attach_enabled_sources(
    all: &[Arc<dyn CompletionSource>],
    buf: &Buffer,
) -> Vec<Arc<dyn CompletionSource>> {
    let mut results = all
        .iter()
        .map(|source| {
            let buf = buf.clone();
            let source = Arc::clone(source);
            tokio::spawn(async move { source.should_attach(&buf).await })
        })
        .collect::<FuturesOrdered<_>>()
        .enumerate();

    let mut sources = Vec::<Arc<dyn CompletionSource>>::new();

    while let Some((idx, res)) = results.next().await {
        if matches!(res, Ok(Ok(true))) {
            sources.push(Arc::clone(&all[idx]));
        }
    }

    sources
}

/// TODO: docs
async fn send_completions(
    sources: &[Arc<dyn CompletionSource>],
    ctx: Arc<CompletionContext>,
    sender: &UnboundedSender<MainMessage>,
    handle: &AsyncHandle,
) -> Vec<JoinHandle<()>> {
    sources
        .iter()
        .map(|source| {
            let ctx = Arc::clone(&ctx);
            let source = Arc::clone(source);
            let sender = sender.clone();
            let mut handle = handle.clone();

            tokio::spawn(async move {
                let completions = source.complete(&ctx).await;
                sender
                    .send(MainMessage::ShowCompletions(completions))
                    .unwrap();
                handle.send().unwrap();
            })
        })
        .collect()
}
