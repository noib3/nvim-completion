use mlua::prelude::LuaResult;
use std::{cmp, num::NonZeroU32};

use crate::completion::CompletionItem;
use crate::settings::ui::menu::MenuAnchor;
use crate::ui::WindowPosition;

/// Figures out where to position the floating window used to display the
/// completion menu.
pub fn get_position(
    completions: &[CompletionItem],
    max_height: Option<NonZeroU32>,
    anchor: &MenuAnchor,
    matched_bytes: u32,
) -> LuaResult<Option<WindowPosition>> {
    let longest_line = completions
        .iter()
        .map(|c| c.line.chars().count() - 1)
        .max()
        .expect("There's at least one completion");

    // The `+ 2` is to pad each item with a space.
    let width: u32 = (longest_line + 2).try_into().unwrap();

    let height = match max_height {
        None => completions.len() as u32,
        Some(height) => cmp::min(u32::from(height), completions.len() as u32),
    };

    let col = match anchor {
        MenuAnchor::Cursor => 0,
        // The `+ 1` is because every completion is formatted with has a
        // leading space.
        MenuAnchor::Match => -i32::try_from(matched_bytes + 1).unwrap(),
    };

    Ok(Some(WindowPosition {
        height,
        width,
        row: 1,
        col,
    }))
}
