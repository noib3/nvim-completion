use tokio::sync::mpsc::UnboundedSender;

use crate::rpc::message::{RpcMessage, RpcNotification};

type Sender = UnboundedSender<RpcMessage>;

pub async fn handle_notification(ntf: RpcNotification, sender: Sender) {
    match sender.send(RpcMessage::Notification(ntf)) {
        Ok(()) => {},
        Err(_) => todo!(),
    };
}
