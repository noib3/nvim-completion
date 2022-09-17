use async_trait::async_trait;
use nvim_oxi::{object, Object};
use serde::de::DeserializeOwned;

use crate::source_bundle::SourceConfigPtr;
use crate::{CompletionList, Document, GenericError, Position};

/// TODO: docs
#[async_trait]
pub trait CompletionSource: Send + Sync + 'static {
    /// The name of the completion source.
    const NAME: &'static str;

    /// TODO: docs
    type Config: Sync + DeserializeOwned;

    /// TODO: docs
    type Error: std::error::Error + Send + Sync + 'static;

    /// TODO: docs
    fn api() -> Object {
        Object::nil()
    }

    /// TODO: docs
    async fn enable(
        &self,
        document: &Document,
        config: &Self::Config,
    ) -> Result<bool, Self::Error>;

    /// TODO: docs
    async fn complete(
        &self,
        document: &Document,
        position: &Position,
        config: &Self::Config,
    ) -> Result<CompletionList, Self::Error>;

    // /// TODO: docs
    // async fn resolve(
    //     &self,
    //     document: &Document,
    //     item: &mut CompletionItem,
    //     config: &Self::Config,
    // ) -> Result<(), Self::Error>;
}

/// TODO: docs
#[async_trait]
pub trait ObjectSafeCompletionSource: Send + Sync + 'static {
    fn api(&self) -> Object;

    fn deser_config(
        &self,
        config: Object,
    ) -> Result<SourceConfigPtr, serde_path_to_error::Error<nvim_oxi::Error>>;

    async fn enable(
        &self,
        document: &Document,
        config: &SourceConfigPtr,
    ) -> Result<bool, GenericError>;

    async fn complete(
        &self,
        document: &Document,
        position: &Position,
        config: &SourceConfigPtr,
    ) -> Result<CompletionList, GenericError>;
}

#[async_trait]
impl<S> ObjectSafeCompletionSource for S
where
    S: CompletionSource,
{
    #[inline]
    fn api(&self) -> Object {
        <Self as CompletionSource>::api()
    }

    fn deser_config(
        &self,
        config: Object,
    ) -> Result<SourceConfigPtr, serde_path_to_error::Error<nvim_oxi::Error>>
    {
        let config: <Self as CompletionSource>::Config = {
            let deserializer = object::Deserializer::new(config);
            serde_path_to_error::deserialize(deserializer)?
            // .map_err(|err| crate::Error::source_deser(err, S::NAME))?
        };

        Ok(SourceConfigPtr::new(config))
    }

    #[inline]
    async fn enable(
        &self,
        document: &Document,
        config: &SourceConfigPtr,
    ) -> Result<bool, GenericError> {
        // Safety: TODO.
        let config: &S::Config = unsafe { config.cast() };

        <Self as CompletionSource>::enable(self, document, config)
            .await
            .map_err(|err| Box::new(err) as _)
    }

    #[inline]
    async fn complete(
        &self,
        document: &Document,
        position: &Position,
        config: &SourceConfigPtr,
    ) -> Result<CompletionList, GenericError> {
        // Safety: TODO.
        let config: &S::Config = unsafe { config.cast() };

        <Self as CompletionSource>::complete(self, document, position, config)
            .await
            .map_err(|err| Box::new(err) as _)
    }
}
