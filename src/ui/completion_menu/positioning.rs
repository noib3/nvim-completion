use mlua::{Lua, Result};
use neovim::Api;

/// TODO: docs
pub enum MenuPosition {
    /// TODO: docs
    Above { width: usize, height: usize },

    /// TODO: docs
    Below { width: usize },
}

/// TODO: docs
pub fn create_menu_window(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    width: usize,
    height: usize,
) -> Result<(usize, MenuPosition)> {
    let (row, position): (isize, MenuPosition);

    // Plan A: Is there space below? -> Put it below.
    // Plan B: Is there space above? -> Put it above.
    // Plan C: Fuck.
    if is_there_space_below(api, height)? {
        row = 1;
        position = MenuPosition::Below { width };
    } else if is_there_space_above(api, height)? {
        row = -isize::try_from(height).unwrap();
        position = MenuPosition::Above { width, height };
    } else {
        // TODO
        unreachable!();
    }

    let opts = lua.create_table_with_capacity(0, 8)?;
    opts.set("relative", "cursor")?;
    opts.set("width", width)?;
    opts.set("height", height)?;
    opts.set("row", row)?;
    opts.set("col", 0)?;
    opts.set("focusable", false)?;
    opts.set("style", "minimal")?;
    opts.set("noautocmd", true)?;

    let winid = api.open_win(bufnr, false, opts)?;
    api.win_set_option(winid, "winhl", "Normal:CompleetMenu")?;
    api.win_set_option(winid, "scrolloff", 0)?;

    Ok((winid, position))
}

fn is_there_space_above(api: &Api, height: usize) -> Result<bool> {
    let screen_line = api.call_function::<u8, usize>("winline", &[])?;

    Ok(height <= screen_line - 1)
}

fn is_there_space_below(api: &Api, height: usize) -> Result<bool> {
    let window_height = api.win_get_height(0)?;
    let screen_line = api.call_function::<u8, usize>("winline", &[])?;

    Ok(height <= window_height - screen_line)
}
