pub struct VirtualText {
    text: Option<String>,
}

impl VirtualText {
    pub fn new() -> Self {
        VirtualText { text: None }
    }

    pub async fn erase(&mut self) {
        self.text = None
    }
}
