use std::collections::HashMap;
use std::sync::Arc;

use nvim_oxi::Object;
use tokio::sync::oneshot;

use super::{CompletionSource, ObjectSafeCompletionSource, SourceEnable};
use crate::pipeline::{MainMessage, MainSender};
use crate::{Buffer, CompletionContext, CompletionItem, Error, GenericError};

pub(crate) type SourceId = &'static str;
pub(crate) type SourceMap = HashMap<SourceId, SourceBundle>;
pub(crate) type SourceVec = Vec<(SourceId, SourceBundle)>;

#[derive(Clone)]
pub(crate) struct SourceBundle {
    source: Arc<dyn ObjectSafeCompletionSource>,
    config: Option<SourceConfigPtr>,
    enable: Option<SourceEnable>,
}

impl<S> From<S> for SourceBundle
where
    S: CompletionSource,
{
    fn from(source: S) -> Self {
        SourceBundle { source: Arc::new(source), config: None, enable: None }
    }
}

impl SourceBundle {
    #[inline]
    pub(crate) fn api(&self) -> Object {
        self.source.api()
    }

    #[inline]
    pub(crate) fn set_config(&mut self, config: Object) -> Result<(), Error> {
        self.config = Some(self.source.deser_config(config)?);
        Ok(())
    }

    #[inline]
    pub(crate) fn set_enable(&mut self, enable: SourceEnable) {
        self.enable = Some(enable);
    }

    #[inline]
    pub(crate) fn is_initialized(&self) -> bool {
        self.config.is_some()
    }

    /// TODO: docs
    #[inline]
    pub(crate) async fn should_attach(
        &self,
        buf: &Buffer,
        sender: &MainSender,
    ) -> bool {
        let source_enable =
            self.source.should_attach(buf, self.config.as_ref().unwrap());

        match self.enable.as_ref().unwrap() {
            SourceEnable::Final(true) => source_enable.await.unwrap_or(false),

            SourceEnable::Depends(fun) => {
                let user_enable = {
                    let (s, r) = oneshot::channel();
                    let buf = buf.nvim_buf();
                    let msg = MainMessage::QueryAttach(fun.clone(), buf, s);
                    sender.send(msg);
                    r
                };

                matches!(
                    futures::join!(source_enable, user_enable),
                    (Ok(true), Ok(true))
                )
            },

            SourceEnable::Final(false) => unreachable! {},
        }
    }

    #[inline]
    pub(crate) async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
    ) -> Result<Vec<CompletionItem>, GenericError> {
        self.source.complete(buf, ctx, self.config.as_ref().unwrap()).await
    }
}

#[derive(Clone)]
pub(crate) struct SourceConfigPtr(*const ());

unsafe impl Send for SourceConfigPtr {}
unsafe impl Sync for SourceConfigPtr {}

impl SourceConfigPtr {
    /// TODO: docs
    #[inline]
    pub(super) fn new<C: 'static>(config: C) -> Self {
        let leaked = Box::leak::<'static>(Box::new(config));
        Self(leaked as *const _ as *const ())
    }

    /// TODO: docs
    #[inline]
    pub(super) const fn cast<C>(&self) -> &C {
        unsafe { &*(self.0.cast::<C>()) }
    }
}
