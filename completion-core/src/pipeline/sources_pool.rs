use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::{FuturesUnordered, StreamExt};
use nvim_oxi::api::Buffer as NvimBuffer;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::completions::CompletionRequest;
use crate::pipeline::{MainMessage, MainSender};
use crate::sources::{SourceBundle, SourceId, SourceVec};
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

#[tokio::main]
pub(crate) async fn sources_pool(
    sources: SourceVec,
    mut receiver: PoolReceiver,
    sender: MainSender,
) {
    // Mapping from buffers to sources attached to that buffer. When a
    // completion request arrives we only compute completion results from the
    // sources attached to the buffer associated w/ that request.
    let mut buf_sources = HashMap::<NvimBuffer, SourceVec>::new();

    // Holds the handles of the tasks currently computing the completions.
    let mut handles = Vec::<JoinHandle<()>>::new();

    while let Some(msg) = receiver.recv().await {
        match msg {
            PoolMessage::AbortAll => {
                handles.drain(..).for_each(|task| task.abort());
            },

            PoolMessage::QueryAttach(buf) => {
                let sources =
                    self::get_enabled_sources(&sources, &buf, &sender).await;

                if !sources.is_empty() {
                    buf_sources.insert(buf.nvim_buf(), sources);
                    sender.send(MainMessage::AttachBuf(buf));
                }
            },

            PoolMessage::QueryCompletions(req) => {
                // We only query the bundles attached to the buffer in the
                // request. We can always unwrap because..
                let sources = buf_sources.get(&req.nvim_buf()).unwrap();

                handles = sources
                    .iter()
                    .map(|(id, bundle)| {
                        let id: SourceId = id;
                        let bundle = bundle.clone();
                        let req = Arc::clone(&req);
                        let sender = sender.clone();

                        tokio::spawn(async move {
                            query_completions(id, bundle, req, &sender).await;
                        })
                    })
                    .collect();
            },
        }
    }
}

/// TODO: docs
async fn get_enabled_sources(
    sources: &[(SourceId, SourceBundle)],
    buf: &Arc<Buffer>,
    sender: &MainSender,
) -> SourceVec {
    let mut results = sources
        .iter()
        .map(|(id, bundle)| {
            let id: SourceId = id;
            let bundle = bundle.clone();
            let buf = Arc::clone(buf);
            let sender = sender.clone();

            tokio::spawn(async move {
                let res = bundle.should_attach(&buf, &sender).await;
                (id, bundle, res)
            })
        })
        .collect::<FuturesUnordered<_>>();

    let mut enabled = Vec::new();

    while let Some(res) = results.next().await {
        match res {
            Ok((id, bundle, true)) => enabled.push((id, bundle)),
            _ => {},
        }
    }

    enabled
}

/// TODO: docs
async fn query_completions(
    source: SourceId,
    bundle: SourceBundle,
    req: Arc<CompletionRequest>,
    sender: &MainSender,
) {
    let completions = match bundle.complete(&req.buf, &req.ctx).await {
        Ok(completions) => completions,

        Err(err) => {
            let msg = MainMessage::CompleteFailed {
                on_source: source,
                with_error: err,
            };
            sender.send(msg);
            return;
        },
    };

    let msg = MainMessage::HandleCompletions((source, req, completions));
    sender.send(msg);
}
