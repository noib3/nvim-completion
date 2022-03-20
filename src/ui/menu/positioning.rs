use mlua::prelude::LuaResult;
use std::cmp;

use crate::completion::CompletionItem;
use crate::settings::ui::menu::{MenuAnchor, MenuSettings};
use crate::ui::WindowPosition;

/// Figures out where to position the floating window used to display the
/// completion menu.
pub fn get_position(
    completions: &[CompletionItem],
    matched_bytes: u32,
    settings: &MenuSettings,
) -> LuaResult<Option<WindowPosition>> {
    let longest_line = completions
        .iter()
        .map(|c| c.line.chars().count() - 1)
        .max()
        .expect("There's at least one completion");

    // The `+ 2` is to pad each item with a space.
    let width: u32 = (longest_line + 2).try_into().unwrap();

    let height = match settings.max_height {
        None => completions.len() as u32,
        Some(height) => cmp::min(u32::from(height), completions.len() as u32),
    };

    let col = match settings.anchor {
        MenuAnchor::Cursor => 0,
        // The `- 1` is because every completion is formatted with a leading
        // space.
        MenuAnchor::Match => -i32::try_from(matched_bytes).unwrap() - 1,
    }
    // If the left edge of the border is present we need to offset it by
    // placing the menu one more column to the left.
    - if settings.border.is_left_edge_set() { 1 } else { 0 };

    Ok(Some(WindowPosition {
        height,
        width,
        row: 1,
        col,
    }))
}
