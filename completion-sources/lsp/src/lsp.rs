use async_trait::async_trait;
use nvim_completion_core::{
    Buffer,
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
    Result,
};
use nvim_oxi::{Dictionary, Function, Object};
use serde::Deserialize;

use super::client_capabilities::client_capabilities;

pub struct Lsp;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {}

#[async_trait]
impl CompletionSource for Lsp {
    const NAME: &'static str = "lsp";

    type Config = Config;

    fn api() -> Object {
        Dictionary::from_iter([(
            "client_capabilities",
            Function::from_fn(client_capabilities),
        )])
        .into()
    }

    async fn should_attach(
        &self,
        _buf: &Buffer,
        _config: &Config,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn complete(
        &self,
        _buf: &Buffer,
        ctx: &CompletionContext,
        _config: &Config,
    ) -> Result<Vec<CompletionItem>> {
        let completions = vec![CompletionItemBuilder::new(format!(
            "{} received {}",
            Self::NAME,
            ctx.ch()
        ))
        .build()];

        Ok(completions)
    }
}
