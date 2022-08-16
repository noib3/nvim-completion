use async_trait::async_trait;
use compleet_core::{CompletionContext, CompletionItem, CompletionSource};
use nvim_oxi::{Dictionary, Function, Object};

use super::client_capabilities::client_capabilities;

pub struct CompleetLsp;

#[async_trait]
impl CompletionSource for CompleetLsp {
    #[inline]
    fn name(&self) -> &'static str {
        "lsp"
    }

    async fn complete(&self, ctx: &CompletionContext) -> Vec<CompletionItem> {
        vec![CompletionItem::new(format!(
            "{} received {}",
            self.name(),
            ctx.ch()
        ))]
    }

    fn api(&self) -> Object {
        Dictionary::from_iter([(
            "client_capabilities",
            Function::from_fn(client_capabilities),
        )])
        .into()
    }
}