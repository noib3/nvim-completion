use async_trait::async_trait;
use nvim_oxi::{object, Object};
use serde::de::DeserializeOwned;

use super::SourceConfigPtr;
use crate::{Buffer, CompletionContext, CompletionItem, GenericError};

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
    async fn should_attach(
        &self,
        _buf: &Buffer,
        _config: &Self::Config,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    /// TODO: docs
    async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
        config: &Self::Config,
    ) -> Result<Vec<CompletionItem>, Self::Error>;
}

/// TODO: docs
#[doc(hidden)]
#[async_trait]
pub trait ObjectSafeCompletionSource: Send + Sync + 'static {
    fn api(&self) -> Object;

    fn deser_config(
        &self,
        config: Object,
    ) -> Result<SourceConfigPtr, crate::Error>;

    async fn should_attach(
        &self,
        buf: &Buffer,
        config: &SourceConfigPtr,
    ) -> Result<bool, GenericError>;

    async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
        config: &SourceConfigPtr,
    ) -> Result<Vec<CompletionItem>, GenericError>;
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
    ) -> Result<SourceConfigPtr, crate::Error> {
        let config: <Self as CompletionSource>::Config = {
            let deserializer = object::Deserializer::new(config);

            serde_path_to_error::deserialize(deserializer)
                .map_err(|err| crate::Error::source_deser(err, S::NAME))?
        };

        Ok(SourceConfigPtr::new(config))
    }

    #[inline]
    async fn should_attach(
        &self,
        buf: &Buffer,
        config: &SourceConfigPtr,
    ) -> Result<bool, GenericError> {
        <Self as CompletionSource>::should_attach(self, buf, config.cast())
            .await
            .map_err(|err| Box::new(err) as _)
    }

    #[inline]
    async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
        config: &SourceConfigPtr,
    ) -> Result<Vec<CompletionItem>, GenericError> {
        <Self as CompletionSource>::complete(self, buf, ctx, config.cast())
            .await
            .map_err(|err| Box::new(err) as _)
    }
}
