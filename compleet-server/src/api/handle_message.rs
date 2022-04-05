use tokio::sync::mpsc::UnboundedSender;

use crate::api::{incoming, outgoing};
use crate::rpc::RpcMessage;

type Sender = UnboundedSender<RpcMessage>;

/// Handles an incoming RPC message.
pub async fn handle_message(msg: RpcMessage, sender: Sender) {
    match msg {
        RpcMessage::Request { .. } => {},

        RpcMessage::Response { .. } => {},

        RpcMessage::Notification { method, params } => {
            match incoming::Notification::try_from((method, params)) {
                Ok(ntf) => self::handle_notification(ntf, sender).await,
                Err(_) => {},
            }
        },
    }
}

/// Handles an incoming notification.
async fn handle_notification(ntf: incoming::Notification, sender: Sender) {
    use incoming::Notification::*;

    match ntf {
        StopTasks => {},

        SendCompletions(_bufnr, _cursor) => {
            let cmp = vec![crate::completion::Completion {
                details: None,
                format: "Okdoc!!".into(),
                text: "Hi".into(),
                hl_ranges: Vec::new(),
                source: "lsp".into(),
                matched_bytes: 1,
            }];

            let not = outgoing::Notification::ServeCompletions(cmp).into();

            match sender.send(not) {
                Ok(()) => {},
                Err(_) => todo!(),
            }
        },
    }
}
