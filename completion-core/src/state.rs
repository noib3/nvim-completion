use std::collections::HashMap;
use std::sync::Arc;

use completion_types::{
    ClientMessage,
    ClientSender,
    Clock,
    CompletionItem,
    CompletionList,
    Document,
    Position,
    Revision,
    SourceBundle,
};
use futures::stream::{FuturesUnordered, StreamExt};
use nvim_oxi::api::Buffer;
use tokio::task::JoinHandle;

use crate::SourceBundleExt;

type RecomputeHandle = JoinHandle<()>;

pub(crate) struct State {
    /// The enabled sources sent from the client.
    sources: Vec<Arc<SourceBundle>>,

    /// Map from [`Buffer`]s to completion sources enabled for that buffer.
    buffer_sources: HashMap<Buffer, Vec<Arc<SourceBundle>>>,

    /// Handles to the tasks recomputing the current completions.
    recompute_tasks: Vec<RecomputeHandle>,

    /// The last revision sent from the client.
    revision: Revision,

    /// A message sender used to communicate with the client.
    client_sender: ClientSender,

    /// Whether..
    is_sending_completions: bool,

    ///
    completions: Vec<Arc<CompletionItem>>,
}

impl State {
    #[inline]
    pub(crate) fn new(
        sources: Vec<SourceBundle>,
        client_sender: ClientSender,
    ) -> Self {
        let sources = sources.into_iter().map(Arc::new).collect();

        Self {
            sources,
            buffer_sources: HashMap::new(),
            recompute_tasks: Vec::new(),
            revision: Revision::default(),
            client_sender,
            is_sending_completions: false,
            completions: Vec::new(),
        }
    }

    /// Queries all the sources to check if they want to attach to `document`.
    ///
    /// If at least one source wants to attach it sends a
    /// [`UiMessage::AttachDocument`](completion_types::UiMessage::AttachDocument)
    /// message to the client.
    pub(crate) async fn query_attach(&mut self, document: Document) {
        let document = Arc::new(document);

        let mut tasks = self
            .sources
            .iter()
            .map(|bundle| {
                let bundle = Arc::clone(bundle);
                let doc = Arc::clone(&document);
                let sender = self.client_sender.clone();

                tokio::spawn(async move {
                    let res = bundle.enable(&doc, &sender).await;
                    (bundle, res)
                })
            })
            .collect::<FuturesUnordered<_>>();

        let mut enabled = Vec::new();

        while let Some(out) = tasks.next().await {
            match out {
                Ok((bundle, Ok(true))) => enabled.push(bundle),

                Ok((bundle, Err(error))) => {
                    self.client_sender.send(
                        ClientMessage::SourceEnableFailed {
                            source: bundle.id,
                            error,
                        },
                    );
                },

                _ => {},
            }
        }

        if !enabled.is_empty() {
            self.buffer_sources.insert(document.buffer(), enabled);
            self.client_sender
                .send(ClientMessage::AttachDocument { document });
        }
    }

    pub(crate) async fn recompute_completions(
        &mut self,
        document: Arc<Document>,
        position: Position,
        revision: Revision,
        clock: Clock,
    ) {
        assert_ne!(self.revision, revision);
        self.revision = revision;

        self.recompute_tasks.drain(..).for_each(|task| task.abort());

        let sources = self.buffer_sources.get(&document.buffer()).unwrap();

        let position = Arc::new(position);

        let mut tasks = sources
            .iter()
            .map(|bundle| {
                let bundle = Arc::clone(bundle);
                let doc = Arc::clone(&document);
                let pos = Arc::clone(&position);
                let clk = clock.clone();

                tokio::spawn(async move {
                    let res = bundle.complete(&doc, &pos).await;
                    (res, bundle, revision, clk)
                })
            })
            .collect::<FuturesUnordered<_>>();

        while let Some(out) = tasks.next().await {
            match out {
                Ok((Ok(list), bundle, revision, clock)) => {
                    self.on_completions_recomputed(
                        list, bundle, revision, clock,
                    )
                    .await
                },

                Ok((Err(error), bundle, _, _)) => {
                    self.client_sender.send(
                        ClientMessage::SourceCompleteFailed {
                            source: bundle.id,
                            error,
                        },
                    );
                },

                Err(_) => {},
            }
        }
    }

    pub(crate) fn send_completions(
        &self,
        revision: Revision,
        from: std::ops::Bound<u32>,
        to: std::ops::Bound<u32>,
    ) {
        assert_eq!(self.revision, revision);

        use std::ops::Bound::*;

        let start = match from {
            Excluded(i) => i as _,
            Included(i) => i.saturating_sub(1) as _,
            Unbounded => 0,
        };

        let end = match to {
            Excluded(i) => i as _,
            Included(i) => i.saturating_sub(1) as _,
            Unbounded => self.completions.len().saturating_sub(1),
        };

        assert!(start <= end);

        let completions = self.completions[start..end]
            .iter()
            .map(Arc::clone)
            .collect::<Vec<_>>();

        if !completions.is_empty() {
            self.client_sender.send(ClientMessage::Completions {
                items: completions,
                from: start as _,
                to: end as _,
                revision,
                clock: Clock::start(),
            })
        }
    }

    pub(crate) fn stop_sending(&mut self, revision: Revision) {
        assert_eq!(self.revision, revision);
        self.is_sending_completions = false;
    }

    async fn on_completions_recomputed(
        &mut self,
        list: CompletionList,
        bundle: Arc<SourceBundle>,
        revision: Revision,
        clock: Clock,
    ) {
        if self.revision != revision {
            return;
        };

        let items = list.items.into_iter().map(Arc::new).collect::<Vec<_>>();

        let items = vec![Arc::new(CompletionItem {
            text: format!("got {} from {}", items.len(), bundle.id),
        })];

        self.client_sender.send(ClientMessage::Completions {
            items,
            from: 0,
            to: 1,
            revision,
            clock,
        })
    }
}
