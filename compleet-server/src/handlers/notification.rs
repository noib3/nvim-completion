use tokio::sync::mpsc::UnboundedSender;

use crate::rpc::message::{RpcMessage, RpcNotification};

type Sender = UnboundedSender<RpcMessage>;

pub async fn handle_notification(ntf: RpcNotification, sender: Sender) {
    // let bytes: Vec<u8> = ntf.into();
    // std::fs::write("/home/noib3/diocan", format!("{:?}", bytes)).unwrap();

    match sender.send(RpcMessage::Notification(ntf)) {
        Ok(()) => {},
        Err(_) => todo!(),
    };
}
