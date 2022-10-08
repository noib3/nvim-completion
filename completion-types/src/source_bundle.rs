use std::sync::Arc;

use crate::{CompletionSource, ObjectSafeCompletionSource, SourceEnable};

pub type SourceId = &'static str;

#[derive(Clone)]
pub struct SourceBundle {
    // #[cfg_attr(feature = "core", visibility::make(pub))]
    pub id: SourceId,

    // #[cfg_attr(feature = "core", visibility::make(pub))]
    pub source: Arc<dyn ObjectSafeCompletionSource>,

    // #[cfg_attr(feature = "core", visibility::make(pub))]
    pub config: Option<SourceConfigPtr>,

    // #[cfg_attr(feature = "core", visibility::make(pub))]
    pub enable: Option<SourceEnable>,
}

impl<S> From<S> for SourceBundle
where
    S: CompletionSource,
{
    #[inline]
    fn from(source: S) -> Self {
        SourceBundle {
            id: S::NAME,
            source: Arc::new(source),
            config: None,
            enable: None,
        }
    }
}

impl From<Box<dyn ObjectSafeCompletionSource>> for SourceBundle {
    #[inline]
    fn from(source: Box<dyn ObjectSafeCompletionSource>) -> Self {
        SourceBundle {
            id: source.name(),
            source: Arc::from(source),
            config: None,
            enable: None,
        }
    }
}

/// TODO: docs
#[derive(Clone)]
pub struct SourceConfigPtr(*const ());

unsafe impl Send for SourceConfigPtr {}
unsafe impl Sync for SourceConfigPtr {}

impl SourceConfigPtr {
    /// TODO: docs
    #[inline(always)]
    pub(super) fn new<C: 'static>(config: C) -> Self {
        let leaked = Box::leak::<'static>(Box::new(config));
        Self(leaked as *const _ as *const ())
    }

    /// TODO: docs
    #[inline(always)]
    pub(super) unsafe fn cast<C>(&self) -> &C {
        &*self.0.cast::<C>()
    }
}
