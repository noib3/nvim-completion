use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use completion_types::{
    Clock,
    CompletionItem,
    CompletionList,
    CoreMessage,
    CoreSender,
    Document,
    GenericError,
    Position,
    Revision,
    ScoredCompletion,
    SourceBundle,
    SourceId,
};
use futures::stream::{FuturesUnordered, StreamExt};
use nvim_oxi::api::Buffer;

use crate::SourceBundleExt;

type RecomputeHandle = tokio::task::JoinHandle<()>;

pub(crate) struct Core {
    state: Arc<Mutex<State>>,
}

impl Clone for Core {
    fn clone(&self) -> Self {
        Self { state: Arc::clone(&self.state) }
    }
}

pub(crate) struct State {
    /// The enabled sources sent from the client.
    sources: Vec<Arc<SourceBundle>>,

    /// A message sender used to communicate with the client.
    sender: CoreSender,

    /// Map from [`Buffer`]s to completion sources enabled for that buffer.
    buffer_sources: HashMap<Buffer, Vec<Arc<SourceBundle>>>,

    /// The last revision sent from the client.
    revision: Revision,

    ///
    recompute_tasks: Vec<RecomputeHandle>,

    /// Whether..
    is_sending_completions: bool,

    ///
    completions: HashMap<SourceId, Vec<Arc<CompletionItem>>>,
}

// impl Completions {
//     #[inline]
//     fn new(ids: &[SourceId]) -> Self {
//         let by_source = ids
//             .into_iter()
//             .map(|id| (*id, Vec::new()))
//             .collect::<HashMap<_, _>>();

//         Self { by_source }
//     }

//     fn clear(&mut self) {
//         self.by_source.values_mut().for_each(|vec| vec.clear());
//     }
// }

impl Core {
    #[inline]
    pub(crate) fn new(sources: Vec<SourceBundle>, sender: CoreSender) -> Self {
        let mut completions = HashMap::new();

        for source in &sources {
            completions.insert(source.id, Vec::new());
        }

        // let completions = sources
        //     .iter()
        //     .map(|bundle| (bundle.id, Vec::new()))
        //     .collect::<HashMap<_, _>>();

        // panic!("completions: {}", completions.len());

        let sources = sources.into_iter().map(Arc::new).collect();

        let state = State {
            sources,
            sender,
            buffer_sources: HashMap::new(),
            revision: Revision::default(),
            recompute_tasks: Vec::new(),
            is_sending_completions: false,
            completions,
        };

        Self { state: Arc::new(Mutex::new(state)) }
    }

    /// Queries all the sources to check if they want to attach to `document`.
    ///
    /// If at least one source wants to attach it sends a
    /// [`UiMessage::AttachDocument`](completion_types::UiMessage::AttachDocument)
    /// message to the client.
    pub(crate) async fn query_attach(&self, document: Document) {
        let document = Arc::new(document);

        let state = &mut *self.state.lock().unwrap();

        let mut tasks = state
            .sources
            .iter()
            .map(|bundle| {
                let bundle = Arc::clone(bundle);
                let doc = Arc::clone(&document);
                let sender = state.sender.clone();

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
                    state.sender.send(CoreMessage::SourceEnableFailed {
                        source: bundle.id,
                        error,
                    });
                },

                _ => {},
            }
        }

        if !enabled.is_empty() {
            state.buffer_sources.insert(document.buffer(), enabled);
            state.sender.send(CoreMessage::AttachDocument { document });
        }
    }

    pub(crate) fn recompute_completions(
        &self,
        document: Arc<Document>,
        position: Position,
        revision: Revision,
        clock: Clock,
    ) {
        let state = &mut *self.state.lock().unwrap();

        assert_ne!(state.revision, revision);

        state.revision = revision;
        state.is_sending_completions = true;
        state.recompute_tasks.drain(..).for_each(|task| task.abort());
        state.completions.clear();

        let sources = state.buffer_sources.get(&document.buffer()).unwrap();

        let position = Arc::new(position);

        state.recompute_tasks = sources
            .iter()
            .map(|bundle| {
                let core = self.clone();
                let bundle = Arc::clone(bundle);
                let doc = Arc::clone(&document);
                let pos = Arc::clone(&position);
                let clk = clock.clone();

                tokio::spawn(async move {
                    match bundle.complete(&doc, &pos).await {
                        Ok(list) => core.on_completions_recomputed(
                            list, bundle, doc, revision, pos, clk,
                        ),

                        Err(err) => core.complete_failed(bundle.id, err),
                    }
                })
            })
            .collect::<Vec<_>>();
    }

    // pub(crate) fn send_completions(
    //     &self,
    //     revision: Revision,
    //     from: std::ops::Bound<u32>,
    //     to: std::ops::Bound<u32>,
    // ) {
    //     // assert_eq!(self.revision, revision);

    //     // use std::ops::Bound::*;

    //     // let start = match from {
    //     //     Excluded(i) => i as _,
    //     //     Included(i) => i.saturating_sub(1) as _,
    //     //     Unbounded => 0,
    //     // };

    //     // let end = match to {
    //     //     Excluded(i) => i as _,
    //     //     Included(i) => i.saturating_sub(1) as _,
    //     //     Unbounded => self.completions.len().saturating_sub(1),
    //     // };

    //     // assert!(start <= end);

    //     // let completions = self.completions[start..end]
    //     //     .iter()
    //     //     .map(Arc::clone)
    //     //     .collect::<Vec<_>>();

    //     // if !completions.is_empty() {
    //     //     self.client_sender.send(CoreMessage::Completions {
    //     //         items: completions,
    //     //         from: start as _,
    //     //         to: end as _,
    //     //         revision,
    //     //         clock: Clock::start(),
    //     //     })
    //     // }
    // }

    pub(crate) fn stop_sending(&self, revision: Revision) {
        let state = &mut *self.state.lock().unwrap();

        assert_eq!(state.revision, revision);
        state.is_sending_completions = false;
        state.recompute_tasks.drain(..).for_each(|task| task.abort());
    }

    fn on_completions_recomputed(
        &self,
        list: CompletionList,
        bundle: Arc<SourceBundle>,
        document: Arc<Document>,
        revision: Revision,
        position: Arc<Position>,
        mut clock: Clock,
    ) {
        clock.time_source();

        let state = &mut *self.state.lock().unwrap();

        if revision != state.revision {
            return;
        };

        // panic!("{}, {}", state.completions.len(), bundle.id);

        *state.completions.entry(bundle.id).or_default() =
            list.items.into_iter().map(Arc::new).collect();

        // let old = state.completions.get_mut(&bundle.id).unwrap();
        // *old = list.items.into_iter().map(Arc::new).collect();

        let all =
            state.completions.values().flatten().cloned().collect::<Vec<_>>();

        let core = self.clone();

        let _ = std::thread::spawn(move || {
            let sorted =
                crate::sort(all, Arc::clone(&document), Arc::clone(&position));

            core.on_completions_sorted(
                sorted,
                revision,
                document.buffer(),
                position,
                clock,
            );
        });
    }

    fn on_completions_sorted(
        &self,
        items: Vec<ScoredCompletion>,
        revision: Revision,
        buffer: Buffer,
        position: Arc<Position>,
        mut clock: Clock,
    ) {
        clock.time_sorting();

        let state = &*self.state.lock().unwrap();

        if revision == state.revision {
            state.sender.send(CoreMessage::Completions {
                items,
                revision,
                buffer,
                position,
                clock,
            });
        }
    }

    #[inline]
    fn complete_failed(&self, source: SourceId, error: GenericError) {
        let state = &*self.state.lock().unwrap();
        state.sender.send(CoreMessage::SourceCompleteFailed { source, error })
    }
}
