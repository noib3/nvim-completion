use async_trait::async_trait;
use nvim_rs::compat::tokio::Compat;
use rmpv::Value;
use tokio::io::Stdout;

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl nvim_rs::Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(
        &self,
        _method: String,
        _args: Vec<Value>,
        _neovim: nvim_rs::Neovim<Self::Writer>,
    ) {
        unimplemented!()
    }

    async fn handle_request(
        &self,
        method: String,
        args: Vec<Value>,
        _neovim: nvim_rs::Neovim<Self::Writer>,
    ) -> Result<Value, Value> {
        match method.as_ref() {
            "ping" => {
                match args[0].as_str().expect("Was expecting a string") {
                    "Neovim says ping!" => Ok(Value::from("Rust says pong!")),
                    _ => Err(Value::from("Idk what that is :(")),
                }
            },
            _ => Err(Value::Nil),
        }
    }
}

#[tokio::main]
async fn main() {
    let (_nvim, io_handler) =
        nvim_rs::create::tokio::new_parent(NeovimHandler {}).await;

    match io_handler.await {
        Ok(_) => {},
        Err(_) => {},
    }
}
