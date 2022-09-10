use std::sync::Arc;
use std::time::Instant;

use super::{CompletionContext, CompletionItem};
use crate::sources::SourceId;

/// Packs together a source identifier, a completion request sent to that
/// source and the response it sent back (i.e. a bunch of completion items).
pub(crate) type CompletionBundle =
    (SourceId, Arc<CompletionRequest>, Vec<CompletionItem>, Vec<usize>);

// TODO: docs
#[derive(Debug)]
pub(crate) struct CompletionRequest {
    pub(crate) buf: Arc<crate::Buffer>,
    pub(crate) ctx: CompletionContext,
    pub(crate) start: Instant,
    pub(crate) rev: RevId,
}

impl CompletionRequest {
    pub fn nvim_buf(&self) -> nvim_oxi::api::Buffer {
        self.buf.nvim_buf()
    }
}

/// Uniquely identifies an edit into a buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RevId {
    pub(crate) buf: nvim_oxi::api::Buffer,
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
