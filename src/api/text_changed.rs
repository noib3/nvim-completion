use mlua::{Lua, Result};
use std::cmp;

use crate::completion::{self, CompletionState};
use crate::ui::UIState;
use crate::Nvim;

/// Executed on every `TextChangedI` event.
pub fn text_changed(
    lua: &Lua,
    completion_state: &mut CompletionState,
    ui_state: &mut UIState,
) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    completion_state.current_line = nvim.get_current_line()?;
    completion_state.bytes_before_cursor = nvim.win_get_cursor(0)?.1;

    completion_state.matched_prefix = completion::get_matched_prefix(
        &completion_state.current_line,
        completion_state.bytes_before_cursor,
    );

    completion_state.completion_items =
        completion::complete(&completion_state.matched_prefix);

    if completion_state.completion_items.is_empty() {
        ui_state.completion_menu.selected_index = None;
        return Ok(());
    }

    // TODO: I don't actually need this bc the completion_menu is hidden on
    // every cursor_moved and every completion_menu.hide() already sets the
    // selected index to None. Maybe I do if I decide it's not the
    // responsability of completion_menu.hide() to reset the selected index.
    if let Some(index) = ui_state.completion_menu.selected_index {
        ui_state.completion_menu.selected_index =
            Some(cmp::min(index, completion_state.completion_items.len() - 1))
    }

    ui_state.completion_menu.show_completions(
        &nvim,
        lua,
        &completion_state.completion_items,
    )?;

    // Only show a completion hint if there's no text in the line beyond the
    // current cursor position
    if completion_state.bytes_before_cursor
        == completion_state.current_line.len()
    {
        let current_row = nvim.win_get_cursor(0)?.0;
        ui_state.completion_hint.set(
            lua,
            &nvim,
            current_row - 1,
            completion_state.bytes_before_cursor,
            &completion_state.completion_items[0].text
                [completion_state.matched_prefix.len()..],
        )?;
    }

    Ok(())
}
