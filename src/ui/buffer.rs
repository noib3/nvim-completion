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

    // TODO: offer some more abstraction over the raw api?
    pub async fn add_highlight(&self, line: i64) {
        self.0
            .add_highlight(-1, "Visual", line, 0, -1)
            .await
            .unwrap();
    }

    pub async fn clear_highlight(&self, line: i64) {
        self.0.clear_highlight(-1, line, line + 1).await.unwrap();
    }
}
