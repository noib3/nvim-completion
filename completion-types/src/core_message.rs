use std::ops::Bound;
use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{Clock, Document, Position, Revision};

pub type CoreReceiver = UnboundedReceiver<CoreMessage>;
pub type CoreSender = UnboundedSender<CoreMessage>;

/// Messages sent from the UI to the core.
#[derive(Debug)]
pub enum CoreMessage {
    /// Let's the core check which sources should attach to this document.
    QueryAttach { document: Document },

    /// Tells the core to recompute its completions...
    RecomputeCompletions {
        document: Arc<Document>,
        position: Position,
        revision: Revision,
        clock: Clock,
    },

    /// TODO: docs
    SendCompletions { revision: Revision, from: Bound<u32>, to: Bound<u32> },

    /// TODO: docs
    StopSending { revision: Revision },
}
