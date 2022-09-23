use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{Clock, CompletionItem, Document, Position, Revision, SourceId};

pub type ClientReceiver = UnboundedReceiver<ClientMessage>;
pub type ClientSender = UnboundedSender<ClientMessage>;

/// Messages sent from the UI to the core.
#[derive(Debug)]
pub enum ClientMessage {
    /// Let's the core check which sources should attach to this document.
    QueryAttach { document: Document },

    /// Tells the core to recompute its completions...
    CompletionRequest { request: CompletionRequest },

    ResolveCompletion {
        document: Arc<Document>,
        item: Arc<CompletionItem>,
        source: SourceId,
        id: Revision,
    },

    /// TODO: docs
    CancelRequest { revision: Revision },
}

#[derive(Debug)]
pub struct CompletionRequest {
    /// The [`Document`] that requested the completions.
    pub document: Arc<Document>,

    /// Contains the current line and the position of the cursor in it.
    pub position: Position,

    /// Uniquely identifies this revision.
    pub id: Revision,

    /// TODO
    pub kind: RequestKind,

    /// Used for performance measurements.
    pub clock: Clock,
}

// user continues typing -> (if only 1 char after the first TypedChar, els all)
// user asks -> RecomputeAll
// after moving cursor in insert mode -> RecomputeAll
// after entering insert mode and typing -> RecomputeAll

#[derive(Debug)]
pub enum RequestKind {
    TypedCharacter(char),
    RecomputeAll,
}
