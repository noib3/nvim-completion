use std::cmp;
use std::num::NonZeroUsize;

use compleet::completion::Completions;
use mlua::{prelude::LuaResult, Lua};

use super::floater::Floater;
use crate::bindings::{api, r#fn};
use crate::settings::ui::menu::MenuSettings;

#[derive(Debug)]
pub struct CompletionMenu {
    /// Number of the buffer used to show the completions. It's created on
    /// once in `CompletionMenu::new` and never changes.
    pub bufnr: u32,

    /// Floating window used to show the completion menu.
    pub floater: Floater,

    /// The index of the currently selected completion item, or `None` if no
    /// completion is selected.
    pub selected_index: Option<usize>,
}

impl CompletionMenu {
    pub fn new(lua: &Lua, settings: &MenuSettings) -> LuaResult<Self> {
        let bufnr = api::create_buf(lua, false, true)?;
        Ok(CompletionMenu {
            bufnr,
            floater: Floater::new(
                lua,
                bufnr,
                &settings.border,
                vec![
                    // TODO: use hlgroup names from `constants.rs`
                    ("FloatBorder", "CompleetMenuBorder"),
                    ("Normal", "CompleetMenu"),
                    ("Search", "None"),
                ],
            )?,
            selected_index: None,
        })
    }
}

impl CompletionMenu {
    /// Fills the buffer with a list of lines.
    pub fn fill(&self, lua: &Lua, lines: Vec<String>) -> LuaResult<()> {
        api::buf_set_lines(lua, self.bufnr, 0, -1, false, lines)
    }

    /// Whether a completion is currently selected.
    pub fn is_item_selected(&self) -> bool { self.selected_index.is_some() }

    /// Selects a new completion. Should only be called if the completion menu
    /// is already open.
    pub fn select(
        &mut self,
        lua: &Lua,
        new_index: Option<usize>,
    ) -> LuaResult<()> {
        let winid = self.floater.id.expect("The menu is open so it has an id");

        match new_index {
            Some(index) => {
                // `api.nvim_win_set_cursor`'s expects a 1-indexed row number.
                let row = (index + 1).try_into().unwrap();
                api::win_set_cursor(lua, winid, row, 0)?;

                // If no completion was previously selected we turn on the
                // `cursorline` option which highlights the row.
                if self.selected_index.is_none() {
                    api::win_set_option(lua, winid, "cursorline", true)?;
                }
            },

            // If no completion is selected we turn `cursorline` off.
            None => api::win_set_option(lua, winid, "cursorline", false)?,
        }

        self.selected_index = new_index;

        Ok(())
    }
}

/// Tries to find a way to position the completion menu relative to the
/// current cursor position.
pub fn find_position(
    lua: &Lua,
    completions: &Completions,
    floater: &Floater,
    max_height: Option<NonZeroUsize>,
) -> LuaResult<Option<(i32, i32, u16, u16)>> {
    // TODO: max of lines below & lines above should be the max.
    let height = match max_height {
        None => completions.len(),
        Some(height) => cmp::min(usize::from(height), completions.len()),
    } as u16;

    let width = completions
        .iter()
        .map(|c| c.format.chars().count())
        .max()
        .expect("There'a at least one completion") as u16;

    let (rows_above, rows_below) = self::rows_above_below_cursor(lua)?;

    // The total height of the completion menu, also counting the top and
    // bottom edges of its border.
    let height_with_borders = height
        + if floater.border_edges[0] { 1 } else { 0 }
        + if floater.border_edges[1] { 1 } else { 0 };

    let row = if height_with_borders <= rows_below {
        1
    } else if height_with_borders <= rows_above {
        -(height_with_borders as i32)
    } else {
        return Ok(None);
    };

    // TODO: respect `settings.ui.menu.anchor`.
    let col = -if floater.border_edges[2] { 1 } else { 0 };

    Ok(Some((row, col, height, width)))
}

/// Returns the number of screen rows above and below the current cursor
/// position.
fn rows_above_below_cursor(lua: &Lua) -> LuaResult<(u16, u16)> {
    let total_rows = api::get_option::<u16>(lua, "lines")?;
    let rows_above = r#fn::screenrow(lua)? - 1;

    Ok((rows_above, total_rows - rows_above - 1))
}
