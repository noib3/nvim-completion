use super::{Buffer, FloatingWindow};

pub struct DetailsPane {
    /// TODO: docs
    _buffer: Option<Buffer>,

    /// TODO: docs
    window: Option<FloatingWindow>,
}

impl DetailsPane {
    pub fn new() -> Self {
        DetailsPane {
            _buffer: None,
            window: None,
        }
    }

    pub async fn hide(&mut self) {
        if let Some(window) = &self.window {
            window.hide().await;
            self.window = None;
        }
    }
}
