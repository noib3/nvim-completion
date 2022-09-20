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
use nvim_oxi::api::Buffer;

use crate::{Result, SourceBundleExt};

type RecomputeHandle = tokio::task::JoinHandle<Result<()>>;

#[derive(Clone)]
pub(crate) struct Core {
    state: Arc<Mutex<State>>,
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

    /// TODO: docs
    pub(crate) fn query_attach(&self, document: Document) -> Result<()> {
        let document = Arc::new(document);

        let state = &*self.state.lock()?;

        for source in state.sources.iter().map(Arc::clone) {
            let core = self.clone();
            let doc = Arc::clone(&document);
            let sender = state.sender.clone();

            tokio::spawn(async move {
                match source.enable(&doc, &sender).await {
                    Ok(true) => core.source_attached(source, doc).unwrap(),

                    Ok(false) => {},

                    Err(error) => {
                        sender.send(CoreMessage::SourceEnableFailed {
                            source: source.id,
                            error,
                        });
                    },
                }
            });
        }

        Ok(())
    }

    /// TODO: docs
    fn source_attached(
        &self,
        source: Arc<SourceBundle>,
        document: Arc<Document>,
    ) -> Result<()> {
        let state = &mut *self.state.lock()?;
        let sources = &mut state.buffer_sources;
        let buffer = document.buffer();

        match sources.get_mut(&buffer) {
            Some(sources) => sources.push(source),

            None => {
                sources.insert(buffer, vec![source]);
                state.sender.send(CoreMessage::AttachDocument { document });
            },
        }

        Ok(())
    }

    pub(crate) fn recompute_completions(
        &self,
        document: Arc<Document>,
        position: Position,
        revision: Revision,
        clock: Clock,
    ) -> Result<()> {
        let state = &mut *self.state.lock()?;

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

        Ok(())
    }

    pub(crate) fn stop_sending(&self, revision: Revision) -> Result<()> {
        let state = &mut *self.state.lock().unwrap();

        assert_eq!(state.revision, revision);
        state.is_sending_completions = false;
        state.recompute_tasks.drain(..).for_each(|task| task.abort());

        Ok(())
    }

    fn on_completions_recomputed(
        &self,
        list: CompletionList,
        bundle: Arc<SourceBundle>,
        document: Arc<Document>,
        revision: Revision,
        position: Arc<Position>,
        mut clock: Clock,
    ) -> Result<()> {
        clock.time_source();

        let state = &mut *self.state.lock()?;

        if revision != state.revision {
            return Ok(());
        };

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

        Ok(())
    }

    fn on_completions_sorted(
        &self,
        items: Vec<ScoredCompletion>,
        revision: Revision,
        buffer: Buffer,
        position: Arc<Position>,
        mut clock: Clock,
    ) -> Result<()> {
        clock.time_sorting();

        let state = &*self.state.lock()?;

        if revision == state.revision {
            state.sender.send(CoreMessage::Completions {
                items,
                revision,
                buffer,
                position,
                clock,
            });
        }

        Ok(())
    }

    #[inline]
    fn complete_failed(
        &self,
        source: SourceId,
        error: GenericError,
    ) -> Result<()> {
        let state = &*self.state.lock()?;
        state.sender.send(CoreMessage::SourceCompleteFailed { source, error });
        Ok(())
    }
}
