use crate::ui::UIState;

pub fn is_completion_hint_visible(ui_state: &UIState) -> bool {
    ui_state.completion_hint.is_visible()
}
