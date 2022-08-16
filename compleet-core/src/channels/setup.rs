use std::sync::Arc;
use std::thread;

use nvim_oxi::{
    self as nvim,
    r#loop::{self, AsyncHandle},
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{CompletionContext, CompletionItem, CompletionSource};

pub(crate) fn setup(
    sources: Vec<Arc<dyn CompletionSource>>,
    ctx_receiver: UnboundedReceiver<Arc<CompletionContext>>,
) -> crate::Result<()> {
    let (sender, mut recv) = mpsc::unbounded_channel::<Vec<CompletionItem>>();

    let handle = r#loop::new_async(move || {
        let mut msgs = Vec::new();

        while let Ok(sources) = recv.try_recv() {
            msgs.push(sources.into_iter().next().unwrap().text);
        }

        nvim::schedule(move |_| {
            let time = std::time::Instant::now();
            for msg in msgs {
                nvim::print!("{msg}, {time:?}",);
            }
            Ok(())
        });

        Ok(())
    })?;

    let _ = thread::spawn(move || {
        sources_pool(&sources, ctx_receiver, sender, handle)
    });

    Ok(())
}

#[tokio::main]
pub(super) async fn sources_pool(
    sources: &[Arc<dyn CompletionSource>],
    mut ctx_receiver: UnboundedReceiver<Arc<CompletionContext>>,
    completions_sender: UnboundedSender<Vec<CompletionItem>>,
    cb_handle: AsyncHandle,
) {
    while let Some(ctx) = ctx_receiver.recv().await {
        let _handles = sources
            .iter()
            .map(|source| {
                let ct = Arc::clone(&ctx);
                let sender = completions_sender.clone();
                let mut handle = cb_handle.clone();
                let source = Arc::clone(source);

                tokio::spawn(async move {
                    let completions = source.complete(&ct).await;
                    sender.send(completions).unwrap();
                    handle.send().unwrap();
                })
            })
            .collect::<Vec<_>>();
    }
}
