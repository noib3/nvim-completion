use async_trait::async_trait;
use nvim_rs::{compat::tokio::Compat, Neovim};
use rmpv::Value;
use tokio::io::Stdout;

use compleet::{
    handle_cursor_moved_i, handle_insert_char_pre, handle_insert_leave,
};

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl nvim_rs::Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(
        &self,
        method: String,
        args: Vec<Value>,
        nvim: Neovim<Self::Writer>,
    ) {
        match method.as_str() {
            "CursorMovedI" => handle_cursor_moved_i(&nvim).await,
            "InsertCharPre" => {
                handle_insert_char_pre(&nvim, &args[0].as_str().unwrap_or(""))
                    .await;
            },
            "InsertLeave" => handle_insert_leave(&nvim).await,
            _ => {},
        }
    }

    async fn handle_request(
        &self,
        method: String,
        args: Vec<Value>,
        _nvim: Neovim<Self::Writer>,
    ) -> Result<Value, Value> {
        match method.as_str() {
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
