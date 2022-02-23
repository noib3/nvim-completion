// use super::{CompletionMenu, DetailsPane, VirtualText};

pub struct UIState {
    pub foo: usize,
    // /// TODO: docs
    // pub completion_menu: CompletionMenu,

    // /// TODO: docs
    // pub details_pane: DetailsPane,

    // /// TODO: docs
    // pub virtual_text: VirtualText,
}

impl UIState {
    pub fn new() -> Self {
        UIState { foo: 0 }
        // UIState {
        //     completion_menu: CompletionMenu::new(),
        //     details_pane: DetailsPane::new(),
        //     virtual_text: VirtualText::new(),
        // }
    }
}
