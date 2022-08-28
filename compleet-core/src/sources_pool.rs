use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::{FuturesOrdered, StreamExt};
use nvim_oxi::api::Buffer as NvimBuffer;
use nvim_oxi::r#loop::AsyncHandle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use super::{MainMessage, MainSender};
use crate::{Buffer, CompletionRequest, CompletionSource};

pub(crate) type PoolSender = mpsc::UnboundedSender<PoolMessage>;
type PoolReceiver = mpsc::UnboundedReceiver<PoolMessage>;

/// Messages sent from the main thread to the thread pool.
#[derive(Debug)]
pub(crate) enum PoolMessage {
    /// TODO: docs
    AbortAll,

    /// TODO: docs
    QueryAttach(Arc<Buffer>),

    /// TODO: docs
    QueryCompletions(Arc<CompletionRequest>),
}

/// TODO: let this thread pool own the sources which are currently stored as a
/// hashmap on the UI thread?
#[tokio::main]
pub(crate) async fn sources_pool(
    sources: Vec<Arc<dyn CompletionSource>>,
    mut receiver: PoolReceiver,
    cb_sender: MainSender,
    mut cb_handle: AsyncHandle,
) {
    // TODO: docs
    //
    // REFACTOR: this should be rethought.
    let mut buf_sources =
        HashMap::<NvimBuffer, Vec<Arc<dyn CompletionSource>>>::new();

    // TODO: docs
    //
    // REFACTOR: this should be rethought.
    let mut handles = Vec::<JoinHandle<()>>::new();

    while let Some(msg) = receiver.recv().await {
        match msg {
            PoolMessage::AbortAll => {
                handles.drain(..).for_each(|task| task.abort());
            },

            PoolMessage::QueryAttach(buf) => {
                let sources =
                    self::filter_enabled_sources(&sources, &buf).await;

                if !sources.is_empty() {
                    buf_sources.insert(buf.nvim_buf().clone(), sources);
                    cb_sender.send(MainMessage::AttachBuf(buf)).unwrap();
                    cb_handle.send().unwrap();
                }
            },

            PoolMessage::QueryCompletions(req) => {
                handles.drain(..).for_each(|task| task.abort());

                // We only query the sources attached to the buffer in the
                // request.
                let sources = buf_sources.get(&req.nvim_buf()).unwrap();

                handles = self::send_completions(
                    sources, req, &cb_sender, &cb_handle,
                )
                .await;
            },
        }
    }
}

/// TODO: docs
async fn filter_enabled_sources(
    all: &[Arc<dyn CompletionSource>],
    buf: &Arc<Buffer>,
) -> Vec<Arc<dyn CompletionSource>> {
    let mut results = all
        .iter()
        .map(|source| {
            let buf = Arc::clone(&buf);
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
    req: Arc<CompletionRequest>,
    sender: &MainSender,
    handle: &AsyncHandle,
) -> Vec<JoinHandle<()>> {
    sources
        .iter()
        .map(|source| {
            let source = Arc::clone(source);
            let req = Arc::clone(&req);
            let sender = sender.clone();
            let mut handle = handle.clone();

            tokio::spawn(async move {
                let completions = source.complete(&req.buf, &req.ctx).await;
                let bundle = (source.name(), req, completions);
                sender.send(MainMessage::HandleCompletions(bundle)).unwrap();
                handle.send().unwrap();
            })
        })
        .collect()
}
