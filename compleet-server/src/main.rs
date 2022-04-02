use tokio::io::{self, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut _stdin = io::stdin();
    let mut stderr = io::stderr();

    let (sender, mut receiver) = mpsc::channel::<String>(100);

    // Wait for new messages on the receiving end of the channel and write them
    // to stderr.
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match stderr.write_all(message.as_bytes()).await {
                Ok(()) => {},
                Err(_) => todo!(),
            };
        }
    });

    // Listen for new lines on stdin, relaying the string to the channel
    // receiver when one arrives.
    let mut buf = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut buf) {
        let line = buf.drain(..).collect();
        let cloned = sender.clone();

        tokio::spawn(async move {
            match cloned.send(line).await {
                Ok(()) => {},
                Err(_) => todo!(),
            }
        });
    }

    Ok(())
}
