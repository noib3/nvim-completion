use async_trait::async_trait;
use completion_types::{CompletionList, CompletionSource, Document, Position};
use nvim_oxi::{Dictionary, Function, Object};
use serde::Deserialize;
use thiserror::Error as ThisError;

use super::client_capabilities::client_capabilities;

pub struct Lsp;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {}

#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct Error(&'static str);

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
impl CompletionSource for Lsp {
    const NAME: &'static str = "lsp";

    type Config = Config;

    type Error = Error;

    fn api() -> Object {
        Dictionary::from_iter([(
            "client_capabilities",
            Function::from_fn(client_capabilities),
        )])
        .into()
    }

    async fn enable(&self, _doc: &Document, _config: &Config) -> Result<bool> {
        Ok(true)
    }

    async fn complete(
        &self,
        _doc: &Document,
        _pos: &Position,
        _config: &Config,
    ) -> Result<CompletionList> {
        Err(Error("AA!"))
    }
}
