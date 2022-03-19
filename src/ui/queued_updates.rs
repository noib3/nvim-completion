#[derive(Debug)]
pub struct QueuedUpdates {
    // TODO: docs
    pub menu_position: Option<super::WindowPosition>,

    // TODO: docs
    pub details_position: Option<super::WindowPosition>,

    // TODO: docs
    pub hinted_index: Option<usize>,
}

impl QueuedUpdates {
    pub fn new() -> Self {
        Self {
            menu_position: None,
            details_position: None,
            hinted_index: None,
        }
    }

    pub fn reset(&mut self) {
        self.menu_position = None;
        self.details_position = None;
        self.hinted_index = None;
    }
}
