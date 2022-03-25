use std::cmp;

use mlua::prelude::LuaResult;
use neovim::Api;

use crate::completion::CompletionItem;
use crate::settings::ui::menu::{MenuAnchor, MenuSettings};
use crate::ui::WindowPosition;

/// Figures out where to position the floating window used to display the
/// completion menu.
pub fn get_position(
    api: &Api,
    completions: &[CompletionItem],
    settings: &MenuSettings,
) -> LuaResult<Option<WindowPosition>> {
    let longest_line = completions
        .iter()
        .map(|c| c.format.chars().count() - 1)
        .max()
        .expect("There's at least one completion");

    // The `+ 2` is to pad each item with a space.
    let width: u32 = (longest_line + 2).try_into().unwrap();

    let height = match settings.max_height {
        None => completions.len() as u32,
        Some(height) => cmp::min(u32::from(height), completions.len() as u32),
    };

    let border = &settings.border;

    let col = match settings.anchor {
        MenuAnchor::Cursor => 0,
        // The `- 1` is because every completion is formatted with a leading
        // space.
        // TODO: picking the first completion is arbitrary.
        MenuAnchor::Match =>
            -i32::try_from(completions[0].matched_bytes).unwrap() - 1,
    }
    // If the left edge of the border is present we need to offset it by
    // placing the menu one more column to the left.
    - if border.has_left_edge() { 1 } else { 0 };

    let (rows_above, rows_below) = get_rows_above_below_cursor(api)?;
    let total_menu_height = height
        + if border.has_bottom_edge() { 1 } else { 0 }
        + if border.has_top_edge() { 1 } else { 0 };

    let row = if rows_below >= total_menu_height {
        1
    } else if rows_above >= total_menu_height {
        -i32::try_from(total_menu_height).unwrap()
    } else {
        return Ok(None);
    };

    Ok(Some(WindowPosition {
        height,
        width,
        row,
        col,
    }))
}

/// Returns the number of screen rows above and below the current cursor
/// position.
fn get_rows_above_below_cursor(api: &Api) -> LuaResult<(u32, u32)> {
    let total_rows = api.get_option::<u32>("lines")?;
    let rows_above =
        api.call_function::<u8, u32>("screenrow", Vec::new())? - 1;

    Ok((rows_above, total_rows - rows_above - 1))
}
