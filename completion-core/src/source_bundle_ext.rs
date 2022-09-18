use async_trait::async_trait;
use completion_types::{
    CompletionList,
    CoreSender,
    Document,
    GenericError,
    Position,
    SourceBundle,
    SourceEnable,
};

#[async_trait]
pub(crate) trait SourceBundleExt {
    async fn enable(
        &self,
        doc: &Document,
        sender: &CoreSender,
    ) -> Result<bool, GenericError>;

    async fn complete(
        &self,
        doc: &Document,
        pos: &Position,
    ) -> Result<CompletionList, GenericError>;
}

#[async_trait]
impl SourceBundleExt for SourceBundle {
    async fn enable(
        &self,
        doc: &Document,
        sender: &CoreSender,
    ) -> Result<bool, GenericError> {
        let config = self.config.as_ref().unwrap();
        let source_enable = self.source.enable(doc, config);

        match self.enable.as_ref().unwrap() {
            SourceEnable::Bool(true) => source_enable.await,

            SourceEnable::Function(fun) => {
                let user_enable = {
                    let fun = fun.clone();
                    let buf = doc.buffer();
                    sender.on_nvim_thread(move || fun.call(buf))
                };

                match futures::join!(source_enable, user_enable) {
                    (Ok(source), Ok(user)) => Ok(source && user),

                    (Err(source), _) => Err(source),

                    (_, Err(user)) => Err(Box::new(user) as _),
                }
            },

            // We assume that all disabled sources have been filtered out
            // before being passed to the core.
            SourceEnable::Bool(false) => unreachable!(),
        }
    }

    #[inline]
    async fn complete(
        &self,
        doc: &Document,
        pos: &Position,
    ) -> Result<CompletionList, GenericError> {
        let config = self.config.as_ref().unwrap();
        self.source.complete(doc, pos, config).await
    }
}
