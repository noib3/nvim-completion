use async_trait::async_trait;
use compleet_core::{
    Buffer,
    CompletionContext,
    CompletionItem,
    CompletionItemBuilder,
    CompletionSource,
    Result,
};
use nvim_oxi::{Dictionary, Function, Object};

use super::client_capabilities::client_capabilities;

pub struct CompleetLsp;

#[async_trait]
impl CompletionSource for CompleetLsp {
    #[inline]
    fn name(&self) -> &'static str {
        "lsp"
    }

    async fn complete(
        &self,
        _buf: &Buffer,
        ctx: &CompletionContext,
    ) -> Result<Vec<CompletionItem>> {
        let completions = vec![CompletionItemBuilder::new(format!(
            "{} received {}",
            self.name(),
            ctx.ch()
        ))
        .build()];

        Ok(completions)
    }

    fn api(&self) -> Object {
        Dictionary::from_iter([(
            "client_capabilities",
            Function::from_fn(client_capabilities),
        )])
        .into()
    }
}
