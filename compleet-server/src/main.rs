use compleet::handlers;
use compleet::rpc::{self, message::RpcMessage};
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stderr = io::stderr();

    let (sender, mut receiver) = mpsc::unbounded_channel::<RpcMessage>();

    // Wait for new messages on the receiving end of the channel and write them
    // to stderr.
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            let msg = match message {
                RpcMessage::Notification(rpc::message::RpcNotification {
                    method,
                    params: _,
                }) => format!(
                    "server received a notification w/ message \"{method}\""
                ),

                _ => todo!(),
            };

            match stderr.write_all(msg.as_bytes()).await {
                Ok(()) => {},
                Err(_) => todo!(),
            };
        }
    });

    // Listen for new messages on stdin.
    let mut queue = Vec::<u8>::new();
    loop {
        match rpc::decode(&mut stdin, &mut queue).await {
            Ok(msg) => match msg {
                RpcMessage::Request(req) => {
                    let sendr = sender.clone();
                    let _ = tokio::spawn(async move {
                        handlers::handle_request(req, sendr).await;
                    });
                },

                RpcMessage::Response(_rsp) => todo!(),

                RpcMessage::Notification(ntf) => {
                    let sender = sender.clone();
                    let _ = tokio::spawn(async move {
                        handlers::handle_notification(ntf, sender).await;
                    });
                },
            },

            Err(_) => todo!(),
        }
    }
}
