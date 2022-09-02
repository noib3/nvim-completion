use std::collections::HashMap;
use std::sync::Arc;

use nvim_oxi::{r#loop::AsyncHandle, Object};
use tokio::sync::oneshot;

use super::{CompletionSource, ObjectSafeCompletionSource, SourceEnable};
use crate::lateinit::LateInit;
use crate::pipeline::{MainMessage, MainSender};
use crate::{Buffer, CompletionContext, CompletionItem, Result};

pub(crate) type SourceId = &'static str;
pub(crate) type SourceMap = HashMap<SourceId, SourceBundle>;

#[derive(Clone)]
pub(crate) struct SourceBundle {
    source: Arc<dyn ObjectSafeCompletionSource>,
    config: Option<SourceConfigPtr>,
    enable: LateInit<SourceEnable>,
}

// TODO: avoid this by making a Sync version of `LateInit`.
unsafe impl Sync for SourceBundle {}

impl<S> From<S> for SourceBundle
where
    S: CompletionSource,
{
    fn from(source: S) -> Self {
        SourceBundle {
            source: Arc::new(source),
            config: None,
            enable: LateInit::default(),
        }
    }
}

impl SourceBundle {
    #[inline]
    pub(crate) fn api(&self) -> Object {
        self.source.api()
    }

    #[inline]
    pub(crate) fn set_config(&mut self, config: Object) -> Result<()> {
        self.config = Some(self.source.deser_config(config)?);
        Ok(())
    }

    #[inline]
    pub(crate) fn set_enable(&mut self, enable: SourceEnable) {
        self.enable.set(enable);
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
        handle: &mut AsyncHandle,
    ) -> Result<bool> {
        let source =
            self.source.should_attach(buf, self.config.as_ref().unwrap());

        let user = match &*self.enable {
            SourceEnable::Final(true) => async { true },

            SourceEnable::Depends(fun) => {
                let (s, r) = oneshot::channel();
                let msg =
                    MainMessage::QueryAttach(fun.clone(), buf.nvim_buf(), s);
                sender.send(msg);
                handle.send();
                // let a = r.await.unwrap();
                // async { a }
                todo!()
            },

            SourceEnable::Final(false) => unreachable!(),
        };

        Ok(true)
    }

    #[inline]
    pub(crate) async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
    ) -> Result<Vec<CompletionItem>> {
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
