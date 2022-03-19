use mlua::prelude::LuaResult;
use neovim::Api;
use std::cmp;

use crate::completion::CompletionItem;
use crate::ui::WindowPosition;

/// Figures out where to position the floating window used to display the
/// completion menu. Returns `None` if the the current window is not big enough
/// (either vertically, horizontally, or both) to contain it.
pub fn get_position(
    api: &Api,
    completions: &[CompletionItem],
    max_height: Option<u32>,
) -> LuaResult<Option<WindowPosition>> {
    let longest_line = completions
        .iter()
        .map(|c| c.line.chars().count() - 1)
        .max()
        .expect("There's at least one completion");

    // TODO: this is one column too wide. Why?
    // The `+ 2` is to pad each item with a space.
    let width: u32 = (longest_line + 2).try_into().unwrap();

    // If the current window is narrower than the desired width of the
    // completion menu we just give up.
    let window_width = api.win_get_width(0)?;
    if window_width < width {
        return Ok(None);
    }

    let height = match max_height {
        None => completions.len() as u32,
        Some(height) => cmp::min(height as usize, completions.len()) as u32,
    };

    // Horizontal policy.
    //
    // If there's enough space after the cursor we make the first column of the
    // completion menu start at the current cursor column, if not we shift it
    // left to make the right edge of the menu touch the right edge of the
    // current window.
    let col = match is_there_space_after(api, width)? {
        (true, _) => 0,
        (false, cols) => -i32::try_from(width - cols).unwrap(),
    };

    // Vertical policy.
    //
    // First we try to display the menu below the cursor, if there's not enough
    // space we try to display it above. If that also fails we give up and
    // return an error.
    let row = if is_there_space_below(api, height)? {
        1
    } else if is_there_space_above(api, height)? {
        -i32::try_from(height).unwrap()
    } else {
        // TODO: a better fallback behaviour might be to check if there's more
        // space above or below, squash the height to that value and place it
        // there.
        return Ok(None);
    };

    Ok(Some(WindowPosition {
        width,
        height,
        row,
        col,
    }))
}

/// Checks if there is enough horizontal space *after* the current cursor
/// position to display a floating window with a specific width. Returns the
/// value of that statement plus the number of columns between the cursor and
/// the right edge of the current window.
fn is_there_space_after(api: &Api, width: u32) -> LuaResult<(bool, u32)> {
    let window_width = api.win_get_width(0)?;
    let screen_col = api.call_function::<u8, u32>("wincol", Vec::new())?;

    Ok((
        width <= window_width - screen_col,
        window_width - screen_col,
    ))
}

/// Checks if there is enough vertical space *above* the current cursor
/// position to display a floating window with a specific height.
fn is_there_space_above(api: &Api, height: u32) -> LuaResult<bool> {
    let screen_line = api.call_function::<u8, u32>("winline", Vec::new())?;

    Ok(height <= screen_line - 1)
}

/// Checks if there is enough vertical space *below* the current cursor
/// position to display a floating window with a specific height.
fn is_there_space_below(api: &Api, height: u32) -> LuaResult<bool> {
    let window_height = api.win_get_height(0)?;
    let screen_line = api.call_function::<u8, u32>("winline", Vec::new())?;

    Ok(height <= window_height - screen_line)
}
