use nvim_rs::{compat::tokio::Compat, Buffer, Neovim};
use rmpv::Value;
use tokio::io::Stdout;

mod utils;
use utils::nvim_echo;

// mod window;
// use window::config::FloatingWindowConfig;

type Nvim = Neovim<Compat<Stdout>>;

pub async fn handle_cursor_moved_i(nvim: &Nvim) {
    nvim_echo(nvim, "Cursor moved", "Normal", false).await;
}

pub async fn handle_insert_char_pre(nvim: &Nvim, char: &str) {
    let config = vec![
        (Value::from("relative"), Value::from("cursor")),
        (Value::from("height"), Value::from(2)),
        (Value::from("width"), Value::from(20)),
        (Value::from("row"), Value::from(1)),
        (Value::from("col"), Value::from(0)),
        (Value::from("style"), Value::from("minimal")),
    ];

    // let config =
    //     FloatingWindowConfig::new(&[("relative", "cursor"), ("height", 2)]);

    nvim.open_win(
        &Buffer::new(Value::from(0), nvim.clone()),
        false,
        Vec::<(Value, Value)>::from(config),
    )
    .await
    .unwrap();

    nvim_echo(nvim, &format!("Inserted char: '{}'", char), "Normal", true)
        .await;
}

pub async fn handle_insert_leave(nvim: &Nvim) {
    nvim_echo(nvim, "Left insert mode", "Normal", false).await;
}
