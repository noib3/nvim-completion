use mlua::{prelude::LuaResult, Table};
use neovim::Api;
use std::cmp;

use crate::ui::WindowPosition;

pub fn get_position(
    api: &Api,
    lines: &[String],
    menu_winid: u32,
    menu_width: u32,
) -> LuaResult<Option<WindowPosition>> {
    let longest_line = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .expect("There's at least one line");

    let width = cmp::min(longest_line, 79) as u32;
    let height = lines.len() as u32;

    // Horizontal policy.
    //
    // First we try to display the details to the right of the completion menu,
    // if there's not enough space we try to display it to the left of it. If
    // that also fails we give up and return an error.
    let col = if is_there_space_after_window(api, menu_winid, menu_width)? {
        menu_width.try_into().unwrap()
    } else if is_there_space_before_window(api, menu_winid, menu_width)? {
        -i32::try_from(width).unwrap()
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
    let row = 0;

    Ok(Some(WindowPosition {
        height,
        width,
        row,
        col,
    }))
}

/// TODO: docs
fn is_there_space_after_window(
    api: &Api,
    winid: u32,
    width: u32,
) -> LuaResult<bool> {
    let window_config = api.win_get_config(winid)?;

    let columns_after_window = api.win_get_width(0)?
        - window_config.get::<_, Table>("col")?.get::<_, u32>(false)?
        - window_config.get::<_, u32>("width")?;

    Ok(width <= columns_after_window)
}

/// TODO: docs
fn is_there_space_before_window(
    api: &Api,
    winid: u32,
    width: u32,
) -> LuaResult<bool> {
    let columns_before_window = api
        .win_get_config(winid)?
        .get::<_, Table>("col")?
        .get::<_, u32>(false)?;

    Ok(width <= columns_before_window)
}
