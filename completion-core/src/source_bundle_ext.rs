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

/// Extension trait for [`SourceBundle`]s.
#[async_trait]
pub(crate) trait SourceBundleExt {
    async fn enable(
        &self,
        document: &Document,
        sender: &CoreSender,
    ) -> Result<bool, GenericError>;

    async fn trigger_characters(
        &self,
        document: &Document,
    ) -> Result<Vec<char>, GenericError>;

    async fn complete(
        &self,
        document: &Document,
        position: &Position,
    ) -> Result<CompletionList, GenericError>;
}

#[async_trait]
impl SourceBundleExt for SourceBundle {
    async fn enable(
        &self,
        document: &Document,
        sender: &CoreSender,
    ) -> Result<bool, GenericError> {
        let config = self.config.as_ref().unwrap();
        let source_enable = self.source.enable(document, config);

        match self.enable.as_ref().unwrap() {
            SourceEnable::Bool(true) => source_enable.await,

            SourceEnable::Function(fun) => {
                let user_enable = {
                    let fun = fun.clone();
                    let buffer = document.buffer();
                    sender.on_nvim_thread(move || fun.call(buffer))
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

    async fn trigger_characters(
        &self,
        document: &Document,
    ) -> Result<Vec<char>, GenericError> {
        let config = self.config.as_ref().unwrap();
        self.source.trigger_characters(document, config).await
    }

    async fn complete(
        &self,
        document: &Document,
        position: &Position,
    ) -> Result<CompletionList, GenericError> {
        let config = self.config.as_ref().unwrap();
        self.source.complete(document, position, config).await
    }
}
