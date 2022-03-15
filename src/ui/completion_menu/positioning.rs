use mlua::{Lua, Result};
use neovim::Api;

/// TODO: docs
#[derive(Debug)]
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

    // If there's not enough space horizontally just give up and return error.
    if !is_there_space_horizontally(api, width)? {
        todo!()
    }

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
        // Check where there's more space.
        // If that space is > 1 make the actual height that, else just give up
        // and return err.
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

/// TODO docs
fn is_there_space_above(api: &Api, height: usize) -> Result<bool> {
    let screen_line = api.call_function::<u8, usize>("winline", &[])?;

    Ok(height <= screen_line - 1)
}

/// TODO docs
fn is_there_space_below(api: &Api, height: usize) -> Result<bool> {
    let window_height = api.win_get_height(0)?;
    let screen_line = api.call_function::<u8, usize>("winline", &[])?;

    Ok(height <= window_height - screen_line)
}

/// TODO docs
fn is_there_space_horizontally(_api: &Api, _width: usize) -> Result<bool> {
    Ok(true)
}
