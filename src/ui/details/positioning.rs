use mlua::prelude::LuaResult;
use neovim::Api;
use std::cmp;

use crate::settings::ui::border::BorderSettings;
use crate::ui::WindowPosition;

pub fn get_position(
    api: &Api,
    lines: &[String],
    border: &BorderSettings,
    menu_winid: u32,
    menu_width: u32,
    menu_border: &BorderSettings,
) -> LuaResult<Option<WindowPosition>> {
    let longest_line = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .expect("There's at least one line");

    let width = cmp::min(longest_line, 79) as u32;
    let height = lines.len() as u32;

    let total_details_width = width
        + if border.has_left_edge() { 1 } else { 0 }
        + if border.has_right_edge() { 1 } else { 0 };

    let (cols_before, cols_after) =
        get_cols_before_after_menu(api, menu_winid, menu_width, menu_border)?;

    // Horizontal policy.
    //
    // First we try to display the details to the right of the completion menu,
    // if there's not enough space we try to display it to the left of it. If
    // that also fails we give up and return `None`.
    let col = if cols_after >= total_details_width {
        i32::try_from(menu_width).unwrap()
            + if menu_border.has_right_edge() { 1 } else { 0 }
    } else if cols_before >= total_details_width {
        -i32::try_from(width).unwrap()
            - if menu_border.has_left_edge() { 1 } else { 0 }
            // TODO: why? do I need this
            - if border.has_left_edge() { 1 } else { 0 }
            - if border.has_right_edge() { 1 } else { 0 }
    } else {
        // TODO: a better fallback behaviour might be to try to place the
        // details window above or below the completion_menu.
        return Ok(None);
    };

    // Vertical policy.
    //
    // The top edge of the details window always lines up with the top edge of
    // the completion menu. This is likely to change in the future as we allow
    // it to be placed above or below the completion menu.
    let row = if menu_border.has_top_edge() { -1 } else { 0 };

    Ok(Some(WindowPosition {
        height,
        width,
        row,
        col,
    }))
}

/// Returns the number of screen columns before and after the completion menu,
/// taking into account the menu's horizontal borders.
fn get_cols_before_after_menu(
    api: &Api,
    menu_winid: u32,
    menu_width: u32,
    menu_border: &BorderSettings,
) -> LuaResult<(u32, u32)> {
    let total_cols = api.get_option::<u32>("columns")?;

    let mut cols_before = api.win_get_position(menu_winid)?.1;

    let cols_after = total_cols
        - cols_before
        - menu_width
        - if menu_border.has_right_edge() { 1 } else { 0 };

    if menu_border.has_left_edge() {
        cols_before -= 1;
    }

    Ok((cols_before, cols_after))
}
