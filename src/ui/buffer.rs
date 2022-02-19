use crate::{Nvim, Writer};

pub struct Buffer(pub nvim_rs::Buffer<Writer>);

impl Buffer {
    pub async fn new(nvim: &Nvim) -> Self {
        Buffer(nvim.create_buf(false, true).await.unwrap())
    }

    pub async fn set_lines(&self, lines: &[String]) {
        self.0
            .set_lines(0, -1, false, lines.to_vec())
            .await
            .unwrap()
    }
}
