use std::sync::Arc;

use crate::{CompletionSource, ObjectSafeCompletionSource, SourceEnable};

pub type SourceId = &'static str;
// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// pub struct SourceId(pub(crate) &'static str);

// impl Display for SourceId {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(self.0)
//     }
// }

// impl From<&'static str> for SourceId {
//     #[inline(always)]
//     fn from(source_name: &'static str) -> Self {
//         Self(source_name)
//     }
// }

// impl SourceId {
//     #[inline(always)]
//     pub fn as_str(&self) -> &str {
//         self.0
//     }
// }

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

// impl SourceBundle {
//     #[inline]
//     pub unsafe fn from_ptr(
//         source: *const dyn ObjectSafeCompletionSource,
//     ) -> Self {
//         SourceBundle {
//             source: Arc::from_raw(source),
//             config: None,
//             enable: None,
//         }
//     }

//     #[inline]
//     pub(crate) fn api(&self) -> Object {
//         self.source.api()
//     }

//     #[inline]
//     pub(crate) fn set_config(&mut self, config: Object) -> Result<(), Error> {
//         self.config = Some(self.source.deser_config(config)?);
//         Ok(())
//     }

//     #[inline]
//     pub(crate) fn set_enable(&mut self, enable: SourceEnable) {
//         self.enable = Some(enable);
//     }

//     #[inline]
//     pub(crate) fn is_initialized(&self) -> bool {
//         self.config.is_some()
//     }

//     /// TODO: docs
//     #[inline]
//     pub(crate) async fn should_attach(
//         &self,
//         id: SourceId,
//         buf: &Buffer,
//         sender: &MainSender,
//     ) -> bool {
//         let source_enable =
//             self.source.should_attach(buf, self.config.as_ref().unwrap());

//         match self.enable.as_ref().unwrap() {
//             SourceEnable::Final(true) => source_enable.await.unwrap_or(false),

//             SourceEnable::Depends(fun) => {
//                 let user_enable = {
//                     let (s, r) = oneshot::channel();
//                     let buf = buf.nvim_buf();
//                     let msg =
//                         MainMessage::QueryAttach(id, fun.clone(), buf, s);
//                     sender.send(msg);
//                     r
//                 };

//                 match futures::join!(source_enable, user_enable) {
//                     // If the source returned an error from its `should_attach`
//                     // function we send a message to the main thread to inform
//                     // the user before returning `false`.
//                     (Err(err), _) => {
//                         let msg = MainMessage::AttachFailed {
//                             on_source: id,
//                             with_error: err,
//                         };
//                         sender.send(msg);
//                         false
//                     },

//                     (Ok(source_res), user_res) => {
//                         source_res && user_res.unwrap()
//                     },
//                 }
//             },

//             SourceEnable::Final(false) => unreachable! {},
//         }
//     }

//     #[inline]
//     pub(crate) async fn complete(
//         &self,
//         doc: &crate::newdiocane::Document,
//         // buf: &Buffer,
//         // ctx: &CompletionContext,
//     ) -> Result<Vec<CompletionItem>, GenericError> {
//         todo!()
//         // self.source.complete(buf, ctx, self.config.as_ref().unwrap()).await
//     }
// }
