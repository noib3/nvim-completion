use nvim::api;
use nvim_oxi as nvim;

/// TODO: docs
pub(super) fn get_drawable_rows() -> nvim::Result<u16> {
    let total = api::get_option::<u16>("lines")?;

    let is_tabline_visible = match api::get_option::<i32>("showtabline")? {
        0 => false,
        1 => api::list_tabpages().len() > 1,
        _ => true,
    };

    let is_statusline_visible = match api::get_option::<i32>("laststatus")? {
        1 => api::list_wins().len() > 1,
        2 | 3 => true,
        _ => false,
    };

    let cmd_height = api::get_option::<u16>("cmdheight")?;

    let rows = total
        - (if is_tabline_visible { 1 } else { 0 })
        - (if is_statusline_visible { 1 } else { 0 })
        - cmd_height;

    Ok(rows)
}

/// TODO: docs
#[inline]
pub(super) fn get_drawable_columns() -> nvim::Result<u16> {
    let total = api::get_option::<u16>("columns")?;

    // TODO: check for gutter columns?

    Ok(total)
}
