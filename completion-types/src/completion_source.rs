use async_trait::async_trait;
use nvim_oxi::Object;
use serde::de::DeserializeOwned;

use crate::source_bundle::SourceConfigPtr;
use crate::{
    CompletionItem,
    CompletionList,
    Document,
    GenericError,
    Position,
    ResolvedProperties,
};

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
    async fn trigger_characters(
        &self,
        document: &Document,
        config: &Self::Config,
    ) -> Result<Vec<char>, Self::Error>;

    /// TODO: docs
    async fn complete(
        &self,
        document: &Document,
        position: &Position,
        config: &Self::Config,
    ) -> Result<CompletionList, Self::Error>;

    /// TODO: docs
    async fn resolve_completion(
        &self,
        _document: &Document,
        _completion: &CompletionItem,
        _config: &Self::Config,
    ) -> Result<Option<ResolvedProperties>, Self::Error> {
        Ok(None)
    }
}

/// TODO: docs
#[async_trait]
pub trait ObjectSafeCompletionSource: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    fn api(&self) -> Object;

    fn deserialize_config(
        &self,
        config: Object,
    ) -> Result<
        SourceConfigPtr,
        serde_path_to_error::Error<nvim_oxi::serde::Error>,
    >;

    async fn enable(
        &self,
        document: &Document,
        config: &SourceConfigPtr,
    ) -> Result<bool, GenericError>;

    async fn trigger_characters(
        &self,
        document: &Document,
        config: &SourceConfigPtr,
    ) -> Result<Vec<char>, GenericError>;

    async fn complete(
        &self,
        document: &Document,
        position: &Position,
        config: &SourceConfigPtr,
    ) -> Result<CompletionList, GenericError>;

    async fn resolve_completion(
        &self,
        document: &Document,
        item: &CompletionItem,
        config: &SourceConfigPtr,
    ) -> Result<Option<ResolvedProperties>, GenericError>;
}

#[async_trait]
impl<S> ObjectSafeCompletionSource for S
where
    S: CompletionSource,
{
    #[inline]
    fn name(&self) -> &'static str {
        S::NAME
    }

    #[inline]
    fn api(&self) -> Object {
        S::api()
    }

    #[inline]
    fn deserialize_config(
        &self,
        config: Object,
    ) -> Result<
        SourceConfigPtr,
        serde_path_to_error::Error<nvim_oxi::serde::Error>,
    > {
        let deserializer = nvim_oxi::serde::Deserializer::new(config);
        serde_path_to_error::deserialize::<_, S::Config>(deserializer)
            .map(SourceConfigPtr::new)
    }

    #[inline]
    async fn enable(
        &self,
        document: &Document,
        config: &SourceConfigPtr,
    ) -> Result<bool, GenericError> {
        // Safety: TODO.
        let config: &S::Config = unsafe { config.cast() };

        S::enable(self, document, config)
            .await
            .map_err(|err| Box::new(err) as _)
    }

    #[inline]
    async fn trigger_characters(
        &self,
        document: &Document,
        config: &SourceConfigPtr,
    ) -> Result<Vec<char>, GenericError> {
        // Safety: see above.
        let config: &S::Config = unsafe { config.cast() };

        S::trigger_characters(self, document, config)
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
        // Safety: see above.
        let config: &S::Config = unsafe { config.cast() };

        S::complete(self, document, position, config)
            .await
            .map_err(|err| Box::new(err) as _)
    }

    #[inline]
    async fn resolve_completion(
        &self,
        document: &Document,
        completion: &CompletionItem,
        config: &SourceConfigPtr,
    ) -> Result<Option<ResolvedProperties>, GenericError> {
        // Safety: see above.
        let config: &S::Config = unsafe { config.cast() };

        S::resolve_completion(self, document, completion, config)
            .await
            .map_err(|err| Box::new(err) as _)
    }
}
