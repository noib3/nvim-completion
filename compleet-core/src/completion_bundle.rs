use std::sync::Arc;
use std::time::Instant;

use crate::completion_source::SourceId;
use crate::{CompletionContext, CompletionItem, Result};

// TODO: docs
#[derive(Debug)]
pub(crate) struct CompletionRequest {
    pub(crate) buf: Arc<crate::Buffer>,
    pub(crate) ctx: CompletionContext,
    pub(crate) start: Instant,
    pub(crate) id: RevId,
}

impl CompletionRequest {
    #[inline]
    pub fn new(
        buf: Arc<crate::Buffer>,
        ctx: CompletionContext,
        start: Instant,
        changedtick: u32,
    ) -> Self {
        let id = RevId { buf: buf.nvim_buf(), changedtick };
        Self { buf, ctx, start, id }
    }

    pub fn rev_id(&self) -> &RevId {
        &self.id
    }

    pub fn nvim_buf(&self) -> nvim_oxi::api::Buffer {
        self.buf.nvim_buf()
    }

    pub fn start(&self) -> &Instant {
        &self.start
    }
}

/// Packs together a source identifier, a completion request sent to the source
/// and the response it sent back (i.e. a bunch of completion items).
pub(crate) type CompletionBundle =
    (SourceId, Arc<CompletionRequest>, Result<Vec<CompletionItem>>);

/// Uniquely identifies an edit into a buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RevId {
    buf: nvim_oxi::api::Buffer,
    changedtick: u32,
}

impl Default for RevId {
    #[inline]
    fn default() -> Self {
        Self { buf: 0.into(), changedtick: 0 }
    }
}

impl RevId {
    pub fn new(buf: nvim_oxi::api::Buffer, changedtick: u32) -> Self {
        Self { buf, changedtick }
    }
}
