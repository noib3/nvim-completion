use compleet::api;
use compleet::rpc::{self, RpcMessage};
use rmp_serde::encode::to_vec;
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stderr = io::stderr();

    let (sender, mut receiver) = mpsc::unbounded_channel::<RpcMessage>();

    // Wait for new Rpc messages on the receiving end of the channel and write
    // them to stderr.
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            // TODO: benchmark `From<RpcMessage>` vs `serde::Serialize`.
            //
            // match stderr.write_all(&Vec::<u8>::from(message)).await {
            match stderr.write_all(&to_vec(&message).unwrap()).await {
                Ok(()) => {},
                Err(_) => todo!(),
            };
        }
    });

    // Listen for new messages on stdin, spawning a new task to handle a
    // successfully decoded msgpack-rpc message.
    loop {
        match rpc::decode(&mut stdin).await {
            Ok(message) => {
                let sendr = sender.clone();
                let _ = tokio::spawn(async move {
                    api::handle_message(message, sendr).await
                });
            },

            Err(_) => {},
        }
    }
}
