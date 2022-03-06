use crate::state::UIState;

pub fn is_completion_menu_visible(ui_state: &UIState) -> bool {
    ui_state.completion_menu.is_visible()
}
