/// The possible ways the completion menu could be placed on the screen
/// relative to the current cursor position.
///
/// ```ignore
///          +------+------+
///          +------+------+
///          +--NW--+--NE--+
///          +------+------+
///          +------+------+
/// the sleazy dog j|
///          +------+------+
///          +------+------+
///          +--SW--+--SE--+
///          +------+------+
///          +------+------+
/// ```
#[derive(Default)]
pub(super) enum MenuPosition {
    NorthWest,
    NorthEast,
    SouthWest,
    #[default]
    SouthEast,
}

/// Used to tell [`CompletionMenu::open_window`] and
/// [`CompletionMenu::close_window] where to position the completion menu's
/// window in the buffer.
pub(super) struct MenuGeometry {
    /// The height of the window.
    pub(super) height: u16,

    /// The width of the window.
    pub(super) width: u16,

    /// Vertical offset between the line the cursor is on and the top of the
    /// completion menu.
    ///
    /// For example:
    ///
    /// * `0` => the first row of the menu will be at the same height as the cursor,
    /// * `1` => the first row of the menu will be below the cursor,
    /// * `-height - 1` => the last row of the menu will be above the cursor.
    pub(super) row: i16,

    /// Horizontal offset between the column the cursor is on and the right
    /// side of the completion menu.
    ///
    /// For example:
    ///
    /// * `0` => the first column of the menu will be aligned with the cursor,
    /// * `-width - 1` => the last column of the menu will be aligned with the cursor.
    pub(super) col: i16,
}

impl MenuGeometry {
    pub(super) fn new(
        desired_height: u16,
        desired_width: u16,
        // desired_position: MenuPosition,
        drawable_rows: u16,
        drawable_columns: u16,
    ) -> Self {
        Self { height: desired_height, width: desired_width, row: 1, col: 0 }
    }
}

#[cfg(test)]
mod tests {}
