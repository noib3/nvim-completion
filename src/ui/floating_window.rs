use nvim_rs::Window;
use rmpv::Value;

use super::Buffer;
use crate::{Nvim, Writer};

pub struct FloatingWindow(Window<Writer>);

impl FloatingWindow {
    pub async fn new(
        nvim: &Nvim,
        buffer: &Buffer,
        width: usize,
        height: usize,
    ) -> Self {
        let config = vec![
            (Value::from("relative"), Value::from("cursor")),
            (Value::from("width"), Value::from(width)),
            (Value::from("height"), Value::from(height)),
            (Value::from("row"), Value::from(1)),
            (Value::from("col"), Value::from(0)),
            (Value::from("focusable"), Value::from(false)),
            (Value::from("style"), Value::from("minimal")),
            (Value::from("noautocmd"), Value::from(true)),
        ];

        FloatingWindow(nvim.open_win(&buffer.0, false, config).await.unwrap())
    }

    pub async fn hide(&self) {
        self.0.hide().await.unwrap();
    }
}
