use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::{FuturesOrdered, StreamExt};
use nvim_oxi::api::Buffer as NvimBuffer;
use nvim_oxi::r#loop::AsyncHandle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::completions::CompletionRequest;
use crate::pipeline::{MainMessage, MainSender};
use crate::sources::{SourceBundle, SourceId, SourceMap};
use crate::Buffer;

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
    sources: SourceMap,
    mut receiver: PoolReceiver,
    cb_sender: MainSender,
    mut cb_handle: AsyncHandle,
) {
    let sources = sources.into_iter().collect::<Vec<_>>();

    // TODO: refactor?
    let mut buf_bundles =
        HashMap::<NvimBuffer, Vec<(SourceId, SourceBundle)>>::new();

    // TODO: refactor?
    let mut handles = Vec::<JoinHandle<()>>::new();

    while let Some(msg) = receiver.recv().await {
        match msg {
            PoolMessage::AbortAll => {
                handles.drain(..).for_each(|task| task.abort());
            },

            PoolMessage::QueryAttach(buf) => {
                let bundles = self::filter_enabled_sources(
                    &sources, &buf, &cb_sender, &cb_handle,
                )
                .await;

                if !bundles.is_empty() {
                    buf_bundles.insert(buf.nvim_buf().clone(), bundles);
                    cb_sender.send(MainMessage::AttachBuf(buf)).unwrap();
                    cb_handle.send().unwrap();
                }
            },

            PoolMessage::QueryCompletions(req) => {
                handles.drain(..).for_each(|task| task.abort());

                // We only query the sources attached to the buffer in the
                // request.
                let bundles = buf_bundles.get(&req.nvim_buf()).unwrap();

                handles = self::send_completions(
                    bundles, req, &cb_sender, &cb_handle,
                )
                .await;
            },
        }
    }
}

/// TODO: refactor, way too much cloning.
async fn filter_enabled_sources(
    all: &[(SourceId, SourceBundle)],
    buf: &Arc<Buffer>,
    sender: &MainSender,
    handle: &AsyncHandle,
) -> Vec<(SourceId, SourceBundle)> {
    let mut results = all
        .iter()
        .map(|(_, bundle)| {
            let bundle = bundle.clone();
            let buf = Arc::clone(buf);
            let sender = sender.clone();
            let mut handle = handle.clone();
            tokio::spawn(async move {
                bundle.should_attach(&buf, &sender, &mut handle).await
            })
        })
        .collect::<FuturesOrdered<_>>()
        .enumerate();

    let mut sources = Vec::new();

    while let Some((idx, res)) = results.next().await {
        if matches!(res, Ok(Ok(true))) {
            sources.push(all[idx].clone());
        }
    }

    sources
}

/// TODO: refactor, way too much cloning.
async fn send_completions(
    bundles: &[(SourceId, SourceBundle)],
    req: Arc<CompletionRequest>,
    sender: &MainSender,
    handle: &AsyncHandle,
) -> Vec<JoinHandle<()>> {
    bundles
        .iter()
        .map(|(name, bundle)| {
            let name: SourceId = name;
            let bundle = bundle.clone();
            let req = Arc::clone(&req);
            let sender = sender.clone();
            let mut handle = handle.clone();

            tokio::spawn(async move {
                let completions = bundle.complete(&req.buf, &req.ctx).await;
                let bundle = (name, req, completions);
                sender.send(MainMessage::HandleCompletions(bundle)).unwrap();
                handle.send().unwrap();
            })
        })
        .collect()
}
