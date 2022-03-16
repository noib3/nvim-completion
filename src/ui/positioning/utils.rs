use mlua::Table;
use neovim::Api;

pub type Result<T> = std::result::Result<T, Error>;

/// TODO: docs
pub enum Error {
    /// TODO: docs
    WinTooNarrow,

    /// TODO: docs
    WinTooShort,

    /// TODO: docs
    Lua(mlua::Error),
}

impl From<mlua::Error> for Error {
    fn from(err: mlua::Error) -> Error {
        Error::Lua(err)
    }
}

/// Checks if there is enough horizontal space *after* the current cursor
/// position to display a floating window with a specific width. Returns the
/// value of that statement plus the number of columns between the cursor and
/// the right edge of the current window.
pub fn is_there_space_after(
    api: &Api,
    width: usize,
) -> mlua::Result<(bool, usize)> {
    let window_width = api.win_get_width(0)?;
    let screen_col = api.call_function::<u8, usize>("wincol", &[])?;

    Ok((
        width <= window_width - screen_col + 1,
        window_width - screen_col + 1,
    ))
}

/// Checks if there is enough vertical space *above* the current cursor
/// position to display a floating window with a specific height.
pub fn is_there_space_above(api: &Api, height: usize) -> mlua::Result<bool> {
    let screen_line = api.call_function::<u8, usize>("winline", &[])?;

    Ok(height <= screen_line - 1)
}

/// Checks if there is enough vertical space *below* the current cursor
/// position to display a floating window with a specific height.
pub fn is_there_space_below(api: &Api, height: usize) -> mlua::Result<bool> {
    let window_height = api.win_get_height(0)?;
    let screen_line = api.call_function::<u8, usize>("winline", &[])?;

    Ok(height <= window_height - screen_line)
}

/// TODO: docs
pub fn is_there_space_after_window(
    api: &Api,
    winid: usize,
    width: usize,
) -> mlua::Result<bool> {
    let window_config = api.win_get_config(winid)?;

    let columns_after_window = api.win_get_width(0)?
        - window_config
            .get::<_, Table>("col")?
            .get::<_, usize>(false)?
        - window_config.get::<_, usize>("width")?;

    Ok(width <= columns_after_window)
}

/// TODO: docs
pub fn is_there_space_before_window(
    api: &Api,
    winid: usize,
    width: usize,
) -> mlua::Result<bool> {
    let columns_before_window = api
        .win_get_config(winid)?
        .get::<_, Table>("col")?
        .get::<_, usize>(false)?;

    Ok(width <= columns_before_window)
}
