use mlua::Lua;
use neovim::Api;

use super::utils;

// #[derive(Debug)]
// pub enum MenuPosition {
//     /// TODO: docs
//     Above { height: usize, width: usize },

//     /// TODO: docs
//     Below { height: usize, width: usize },
// }

/// TODO: docs
pub fn create_floatwin(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    width: usize,
    height: usize,
) -> super::Result<(usize, (usize, usize))> {
    // If the current window is narrower than the desired width of the
    // completion menu we just give up.
    let window_width = api.win_get_width(0)?;
    if window_width < width {
        return Err(super::Error::WinTooNarrow);
    }

    // Horizontal policy.
    //
    // If there's enough space after the cursor we make the first column of the
    // completion menu start at the current cursor column, if not we shift it
    // left to make the right edge of the menu touch the right edge of the
    // current window.
    let col = match utils::is_there_space_after(api, width)? {
        (true, _) => 0,
        (false, cols) => -isize::try_from(width - cols).unwrap(),
    };

    // Vertical policy.
    //
    // First we try to display the menu below the cursor, if there's not enough
    // space we try to display it above. If that also fails we give up and
    // return an error.
    let row = if utils::is_there_space_below(api, height)? {
        1
    } else if utils::is_there_space_above(api, height)? {
        -isize::try_from(height).unwrap()
    } else {
        // TODO: a better fallback behaviour might be to check if there's more
        // space above or below, squash the height to that value and place it
        // there.
        return Err(super::Error::WinTooShort);
    };

    let opts = lua.create_table_with_capacity(0, 8)?;
    opts.set("relative", "cursor")?;
    opts.set("width", width)?;
    opts.set("height", height)?;
    opts.set("row", row)?;
    opts.set("col", col)?;
    opts.set("focusable", false)?;
    opts.set("style", "minimal")?;
    opts.set("noautocmd", true)?;

    let winid = api.open_win(bufnr, false, opts)?;
    api.win_set_option(winid, "winhl", "Normal:CompleetMenu")?;
    api.win_set_option(winid, "scrolloff", 0)?;

    Ok((winid, (width, height)))
}
