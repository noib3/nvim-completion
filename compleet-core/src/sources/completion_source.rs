use async_trait::async_trait;
use nvim_oxi::{object, Object};
use serde::de::DeserializeOwned;

use super::SourceConfigPtr;
use crate::{Buffer, CompletionContext, CompletionItem, Result};

/// TODO: docs
#[async_trait]
pub trait CompletionSource: Send + Sync + 'static {
    /// The name of the completion source.
    const NAME: &'static str;

    /// TODO: docs
    type Config: Sync + DeserializeOwned;

    /// TODO: docs
    fn api() -> Object {
        Object::nil()
    }

    /// TODO: docs
    async fn should_attach(
        &self,
        buf: &Buffer,
        config: &Self::Config,
    ) -> Result<bool>;

    /// TODO: docs
    async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
        config: &Self::Config,
    ) -> Result<Vec<CompletionItem>>;
}

/// TODO: docs
#[async_trait]
pub(crate) trait ObjectSafeCompletionSource:
    Send + Sync + 'static
{
    fn deser_config(&self, config: Object) -> Result<SourceConfigPtr>;

    // fn id(&self) -> SourceId;

    fn api(&self) -> Object;

    async fn should_attach(
        &self,
        buf: &Buffer,
        config: &SourceConfigPtr,
    ) -> Result<bool>;

    async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
        config: &SourceConfigPtr,
    ) -> Result<Vec<CompletionItem>>;
}

#[async_trait]
impl<S> ObjectSafeCompletionSource for S
where
    S: CompletionSource,
{
    fn deser_config(&self, config: Object) -> Result<SourceConfigPtr> {
        let config: <Self as CompletionSource>::Config = {
            let deserializer = object::Deserializer::new(config);
            serde_path_to_error::deserialize(deserializer)?
        };

        Ok(SourceConfigPtr::new(config))
    }

    // #[inline]
    // fn id(&self) -> SourceId {
    //     <Self as CompletionSource>::NAME
    // }

    #[inline]
    fn api(&self) -> Object {
        <Self as CompletionSource>::api()
    }

    #[inline]
    async fn should_attach(
        &self,
        buf: &Buffer,
        config: &SourceConfigPtr,
    ) -> Result<bool> {
        <Self as CompletionSource>::should_attach(self, buf, config.cast())
            .await
    }

    #[inline]
    async fn complete(
        &self,
        buf: &Buffer,
        ctx: &CompletionContext,
        config: &SourceConfigPtr,
    ) -> Result<Vec<CompletionItem>> {
        <Self as CompletionSource>::complete(self, buf, ctx, config.cast())
            .await
    }
}
