use std::cmp;
use std::num::NonZeroUsize;

use bindings::{api, r#fn};
use mlua::{prelude::LuaResult, Lua};
use sources::prelude::Completions;

use super::floater::Floater;
use crate::constants::hlgroups::ui;
use crate::settings::ui::menu::MenuSettings;

#[derive(Debug, Default)]
pub struct CompletionMenu {
    /// Number of the buffer used to show the completions. It's created on
    /// once in `CompletionMenu::new` and never changes.
    pub bufnr: u16,

    /// Floating window used to show the completion menu.
    pub floater: Floater,

    /// TODO: docs
    mc_nsid: u32,

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
                    ("FloatBorder", ui::MENU_BORDER),
                    ("Normal", ui::MENU),
                    ("Search", "None"),
                ],
            )?,
            mc_nsid: api::create_namespace(lua, "compleet_matched_chars")?,
            selected_index: None,
        })
    }
}

impl CompletionMenu {
    /// Fills the buffer with a list of lines.
    pub fn fill(&self, lua: &Lua, completions: &Completions) -> LuaResult<()> {
        let lines = completions
            .iter()
            .map(|completion| completion.format.clone())
            .collect::<Vec<String>>();

        api::buf_set_lines(lua, self.bufnr, 0, -1, false, lines)?;

        // Highlight the matching characters of every completion item.
        let mut id = 0u16;
        let opts = lua.create_table_with_capacity(0, 4)?;
        for (row, completion) in completions.iter().enumerate() {
            for range in &completion.matched_bytes {
                id += 1;
                opts.set("id", id)?;
                opts.set("end_row", row)?;
                opts.set("end_col", range.end + 1)?;
                opts.set("hl_group", ui::MENU_MATCHING)?;
                api::buf_set_extmark(
                    lua,
                    self.bufnr,
                    self.mc_nsid,
                    row as u32,
                    range.start as u32 + 1,
                    opts.clone(),
                )?;
            }
        }

        Ok(())
    }

    /// TODO: docs.
    pub fn insert(
        &self,
        lua: &Lua,
        completions: &Completions,
        index: i32,
    ) -> LuaResult<()> {
        let lines = completions
            .iter()
            .map(|completion| completion.format.clone())
            .collect::<Vec<String>>();

        api::buf_set_lines(lua, self.bufnr, index as u32, index, false, lines)
    }

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
                // `api.nvim_win_set_cursor` expects a 1-indexed row number.
                let row = (index + 1).try_into().unwrap();
                api::win_set_cursor(lua, winid, row, 0)?;

                // If no completion was previously selected we turn on the
                // `cursorline` option which highlights the current line.
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
