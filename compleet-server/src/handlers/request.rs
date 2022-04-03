use tokio::sync::mpsc::UnboundedSender;

use crate::rpc::message::{RpcMessage, RpcRequest};

type Sender = UnboundedSender<RpcMessage>;

pub async fn handle_request(req: RpcRequest, sender: Sender) { todo!() }
