use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stderr = io::stderr();
    let mut i = 0;

    loop {
        stderr.write_all(format!("{i}").as_bytes()).await?;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        i += 1;
    }
}
