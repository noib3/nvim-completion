#[derive(Debug)]
pub struct DrawInstructions {
    pub menu_position: Option<super::WindowPosition>,

    pub hinted_index: Option<usize>,
}

impl DrawInstructions {
    pub fn new() -> Self {
        Self {
            menu_position: None,
            hinted_index: None,
        }
    }

    pub fn reset(&mut self) {
        self.menu_position = None;
        self.hinted_index = None;
    }
}
