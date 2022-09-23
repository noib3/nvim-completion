use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use completion_types::{
    Clock,
    CompletionItem,
    CompletionList,
    CompletionRequest,
    CoreMessage,
    CoreSender,
    Document,
    GenericError,
    RequestKind,
    ResolvedProperties,
    Revision,
    ScoredCompletion,
    SourceBundle,
    SourceId,
};
use nvim_oxi::api::Buffer;

use crate::{Result, SourceBundleExt};

type IsComplete = bool;
type TriggerCharacters = Vec<char>;
type AttachedSource = (Arc<SourceBundle>, TriggerCharacters);
type RecomputeHandle = tokio::task::JoinHandle<Result<()>>;
type ResolveHandle = tokio::task::JoinHandle<Result<()>>;

#[derive(Clone)]
pub(crate) struct State {
    inner: Arc<Mutex<StateInner>>,
}

pub(crate) struct StateInner {
    /// The enabled sources sent from the client.
    sources: Vec<Arc<SourceBundle>>,

    /// A message sender used to communicate with the client.
    sender: CoreSender,

    /// Map from [`Buffer`]s to completion sources enabled for that buffer.
    buffer_sources: HashMap<Buffer, Vec<AttachedSource>>,

    /// The last revision sent from the client.
    revision: Revision,

    /// TODO: docs
    recompute_tasks: Vec<RecomputeHandle>,

    /// TODO: docs
    resolve_tasks: Vec<ResolveHandle>,

    /// Whether..
    is_sending_completions: bool,

    /// TODO: docs
    completions:
        HashMap<SourceId, (Vec<Arc<CompletionItem>>, IsComplete, Revision)>,
}

impl State {
    /// TODO: refactor
    #[inline]
    pub(crate) fn new(sources: Vec<SourceBundle>, sender: CoreSender) -> Self {
        let completions = sources
            .iter()
            .map(|source| {
                (source.id, (Vec::new(), false, Revision::default()))
            })
            .collect::<HashMap<_, _>>();

        // panic!("completions: {}", completions.len());

        let sources = sources.into_iter().map(Arc::new).collect();

        let state = StateInner {
            sources,
            sender,
            buffer_sources: HashMap::new(),
            revision: Revision::default(),
            recompute_tasks: Vec::new(),
            resolve_tasks: Vec::new(),
            is_sending_completions: false,
            completions,
        };

        Self { inner: Arc::new(Mutex::new(state)) }
    }

    /// TODO: docs
    pub(crate) fn query_attach(&self, document: Document) -> Result<()> {
        let document = Arc::new(document);

        let state = &*self.inner.lock()?;

        for source in state.sources.iter().map(Arc::clone) {
            let cloned = self.clone();
            let doc = Arc::clone(&document);
            let sender = state.sender.clone();

            tokio::spawn(async move {
                match source.enable(&doc, &sender).await {
                    Ok(true) => {
                        let trigger_chars =
                            source.trigger_characters(&doc).await.unwrap();

                        cloned
                            .source_attached(source, trigger_chars, doc)
                            .unwrap()
                    },

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
        trigger_chars: TriggerCharacters,
        document: Arc<Document>,
    ) -> Result<()> {
        let state = &mut *self.inner.lock()?;
        let sources = &mut state.buffer_sources;
        let buffer = document.buffer();

        match sources.get_mut(&buffer) {
            Some(sources) => sources.push((source, trigger_chars)),

            None => {
                sources.insert(buffer, vec![(source, trigger_chars)]);
                state.sender.send(CoreMessage::AttachDocument { document });
            },
        }

        Ok(())
    }

    /// TODO: docs
    pub(crate) fn recompute_completions(
        &self,
        request: CompletionRequest,
    ) -> Result<()> {
        let state = &mut *self.inner.lock()?;
        let request = Arc::new(request);

        state.revision = request.id;
        state.is_sending_completions = true;
        state.recompute_tasks.drain(..).for_each(|task| task.abort());
        state.resolve_tasks.drain(..).for_each(|task| task.abort());

        let mut cached_completions = Vec::new();

        for (source, trigger_chars) in
            state.buffer_sources.get(&request.document.buffer()).unwrap()
        {
            let (items, is_complete, revision) =
                state.completions.get_mut(&source.id).unwrap();

            assert_ne!(state.revision, *revision);

            if !source_should_recompute(&request, *is_complete, trigger_chars)
            {
                // If the previous completion results are still valid we just
                // update the revision.
                *revision = request.id;
                cached_completions.extend(items.iter().map(Arc::clone));
            } else {
                let cloned = self.clone();
                let source = Arc::clone(source);
                let req = Arc::clone(&request);

                let handle = tokio::spawn(async move {
                    match source.complete(&req.document, &req.position).await {
                        Ok(list) => cloned
                            .on_completions_recomputed(list, source.id, req),

                        Err(err) => cloned.complete_failed(source.id, err),
                    }
                });

                state.recompute_tasks.push(handle);
            }
        }

        if !cached_completions.is_empty() {
            let mut clock = request.clock.clone();
            clock.time_source_finished();

            let state = self.clone();
            let _ = std::thread::spawn(move || {
                let sorted = crate::sort(cached_completions, &request);
                state.on_completions_sorted(sorted, request, clock).unwrap();
            });
        }

        Ok(())
    }

    /// TODO: docs
    fn on_completions_recomputed(
        &self,
        list: CompletionList,
        source: SourceId,
        request: Arc<CompletionRequest>,
    ) -> Result<()> {
        let mut clock = request.clock.clone();
        clock.time_source_finished();

        let state = &mut *self.inner.lock()?;

        if request.id != state.revision {
            return Ok(());
        };

        let (current, is_complete, revision) =
            state.completions.get_mut(&source).unwrap();

        *is_complete = list.is_complete;
        *revision = request.id;
        *current = list.items.into_iter().map(Arc::new).collect();

        let completions = state
            .completions
            .values()
            .flat_map(|(items, _is_complete, revision)| {
                (*revision == state.revision)
                    .then_some(&**items)
                    .unwrap_or(&[])
            })
            .map(Arc::clone)
            .collect::<Vec<_>>();

        let state = self.clone();

        let _ = std::thread::spawn(move || {
            let sorted = crate::sort(completions, &request);
            state.on_completions_sorted(sorted, request, clock).unwrap();
        });

        Ok(())
    }

    /// TODO: docs
    fn on_completions_sorted(
        &self,
        items: Vec<ScoredCompletion>,
        request: Arc<CompletionRequest>,
        mut clock: Clock,
    ) -> Result<()> {
        clock.time_completions_sorted();

        let state = &*self.inner.lock()?;

        if request.id == state.revision && state.is_sending_completions {
            state.sender.send(CoreMessage::Completions {
                items,
                request,
                clock,
            });
        }

        Ok(())
    }

    /// TODO: docs
    pub(crate) fn resolve_completion(
        &self,
        document: Arc<Document>,
        item: Arc<CompletionItem>,
        source_id: SourceId,
        revision: Revision,
    ) -> Result<()> {
        let state = &mut *self.inner.lock()?;

        assert_eq!(state.revision, revision);

        let source = state
            .buffer_sources
            .get(&document.buffer())
            .as_ref()
            .unwrap()
            .iter()
            .find_map(|(source, _trigger_chars)| {
                (source.id == source_id).then_some(source)
            })
            .map(Arc::clone)
            .unwrap();

        let cloned = self.clone();

        let resolve_handle = tokio::spawn(async move {
            match source.resolve_completion(&document, &item).await {
                Ok(Some(properties)) => {
                    cloned.resolved_completion(properties, item, revision)
                },
                Ok(None) => Ok(()),
                Err(err) => cloned.resolve_failed(source.id, err),
            }
        });

        state.resolve_tasks.push(resolve_handle);

        Ok(())
    }

    /// TODO: docs
    fn resolved_completion(
        &self,
        properties: ResolvedProperties,
        item: Arc<CompletionItem>,
        id: Revision,
    ) -> Result<()> {
        let state = &*self.inner.lock()?;

        if id == state.revision {
            state.sender.send(CoreMessage::ResolvedCompletion {
                properties,
                item,
                id,
            });
        }

        Ok(())
    }

    /// TODO: docs
    pub(crate) fn stop_sending(&self, revision: Revision) -> Result<()> {
        let state = &mut *self.inner.lock().unwrap();

        assert_eq!(state.revision, revision);
        state.is_sending_completions = false;
        state.recompute_tasks.drain(..).for_each(|task| task.abort());

        Ok(())
    }

    /// TODO: docs
    fn complete_failed(
        &self,
        source: SourceId,
        error: GenericError,
    ) -> Result<()> {
        let state = &*self.inner.lock()?;
        state.sender.send(CoreMessage::SourceCompleteFailed { source, error });
        Ok(())
    }

    /// TODO: docs
    fn resolve_failed(
        &self,
        source: SourceId,
        error: GenericError,
    ) -> Result<()> {
        let state = &*self.inner.lock()?;
        state.sender.send(CoreMessage::SourceCompleteFailed { source, error });
        Ok(())
    }
}

/// TODO: docs
#[inline]
fn source_should_recompute(
    request: &CompletionRequest,
    is_complete: IsComplete,
    trigger_chars: &[char],
) -> bool {
    match request.kind {
        RequestKind::RecomputeAll => true,
        RequestKind::TypedCharacter(ch) => {
            trigger_chars.contains(&ch) || !is_complete
        },
    }
}
