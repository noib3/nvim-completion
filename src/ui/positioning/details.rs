use mlua::Lua;
use neovim::Api;

use super::{utils, WindowPosition};

// #[derive(Debug)]
// pub enum DetailsPosition {
//     After { width: usize },
//
//     Before { width: usize, height: usize },
// }

/// TODO: docs
pub fn create_floatwin(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    width: usize,
    height: usize,
    menu_winid: usize,
    menu_position: &WindowPosition, // (width, height)
) -> super::Result<usize> {
    let menu_width = menu_position.width;

    // Horizontal policy.
    //
    // First we try to display the details to the right of the completion menu,
    // if there's not enough space we try to display it to the left of it. If
    // that also fails we give up and return an error.
    let (col, anchor) =
        if utils::is_there_space_after_window(api, menu_winid, menu_width)? {
            (menu_width, "NW")
        } else if utils::is_there_space_before_window(
            api, menu_winid, menu_width,
        )? {
            (0, "NE")
        } else {
            // TODO: a better fallback behaviour might be to try to place the
            // details window above or below the completion_menu.
            return Err(super::Error::WinTooNarrow);
        };

    // Vertical policy.
    //
    // The top edge of the details window always lines up with the top edge of
    // the completion menu. This is likely to change in the future as we allow
    // it to be placed above or below the completion menu.
    let row: usize = 0;

    let config = lua.create_table_with_capacity(0, 8)?;
    config.set("relative", "win")?;
    config.set("win", menu_winid)?;
    config.set("anchor", anchor)?;
    config.set("width", width)?;
    config.set("height", height)?;
    config.set("row", row)?;
    config.set("col", col)?;
    config.set("focusable", false)?;
    config.set("style", "minimal")?;
    config.set("noautocmd", true)?;

    let winid = api.open_win(bufnr, false, config)?;
    api.win_set_option(winid, "winhl", "Normal:CompleetDetails")?;
    api.win_set_option(winid, "scrolloff", 0)?;

    Ok(winid)
}
