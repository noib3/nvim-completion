use crate::state::UIState;

pub fn is_completion_item_selected(ui_state: &UIState) -> bool {
    ui_state.completion_menu.selected_index.is_some()
}
