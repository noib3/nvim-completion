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
    nsid: u16,

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
            nsid: api::create_namespace(lua, "compleet/menu")?,
            selected_index: None,
        })
    }
}

impl CompletionMenu {
    /// Fills the buffer with a list of completions.
    pub fn fill(
        &self,
        lua: &Lua,
        completions: &mut Completions,
    ) -> LuaResult<()> {
        let lines = completions
            .iter_mut()
            .map(|completion| completion.format())
            .collect::<Vec<String>>();

        api::buf_set_lines(lua, self.bufnr, 0, -1, false, lines)

        // buffer.set_lines(lua, LineSelect::All, lines)
    }

    /// Highlights the completions.
    pub fn highlight(
        &self,
        lua: &Lua,
        completions: &Completions,
        matched_bytes: usize,
    ) -> LuaResult<()> {
        let mc_opts = lua.create_table_with_capacity(0, 4)?;
        mc_opts.set("hl_group", ui::MENU_MATCHING)?;
        mc_opts.set("priority", 101)?;

        let hl_opts = lua.create_table_with_capacity(0, 4)?;
        hl_opts.set("priority", 100)?;

        for (row, completion) in completions.iter().enumerate() {
            // Highlight the matching characters of every completion item.
            let offset = completion.text_byte_offset();
            mc_opts.set("end_row", row)?;
            mc_opts.set("end_col", offset + matched_bytes)?;
            api::buf_set_extmark(
                lua,
                self.bufnr,
                self.nsid,
                row as u16,
                offset as u16,
                mc_opts.clone(),
            )?;

            // Set the highlight groups of the completion item.
            for (range, hl_group) in completion.hl_ranges() {
                hl_opts.set("end_row", row)?;
                hl_opts.set("end_col", range.end)?;
                hl_opts.set("hl_group", hl_group.to_string())?;
                api::buf_set_extmark(
                    lua,
                    self.bufnr,
                    self.nsid,
                    row as u16,
                    range.start as u16,
                    hl_opts.clone(),
                )?;
            }
        }

        Ok(())
    }

    /// TODO: docs.
    pub fn insert(
        &self,
        lua: &Lua,
        completions: &mut Completions,
        index: i32,
    ) -> LuaResult<()> {
        let lines = completions
            .iter_mut()
            .map(|completion| completion.format().clone())
            .collect::<Vec<String>>();

        api::buf_set_lines(lua, self.bufnr, index, index, false, lines)

        // buffer.set_lines(lua, LineSelect::Single(index), lines)
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
    completions: &mut Completions,
    floater: &Floater,
    max_height: Option<NonZeroUsize>,
) -> LuaResult<Option<(i32, i32, u16, u16)>> {
    // TODO: max of lines below & lines above should be the max.
    let height = match max_height {
        None => completions.len(),
        Some(height) => cmp::min(usize::from(height), completions.len()),
    } as u16;

    let width = completions
        .iter_mut()
        .map(|c| c.len())
        .max()
        .expect("There'a at least one completion") as u16;

    let (rows_above, rows_below) = self::rows_above_below_cursor(lua)?;

    // The total height of the completion menu, also counting the top and
    // bottom edges of its border.
    let total_height = height
        + if floater.border_edges[0] { 1 } else { 0 }
        + if floater.border_edges[1] { 1 } else { 0 };

    let row = if total_height <= rows_below {
        1
    } else if total_height <= rows_above {
        -(total_height as i32)
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
    let lines = api::get_option::<u16>(lua, "lines")?;
    let cmdheight = api::get_option::<u8>(lua, "cmdheight")?;
    let laststatus = api::get_option::<u8>(lua, "laststatus")?;
    let showtabline = api::get_option::<u8>(lua, "showtabline")?;

    let (screenrow, _) = crate::utils::get_screen_cursor(lua)?;

    let statuslineoffset: u8 = match laststatus {
        0 => 0,
        1 => {
            let is_split =
                r#fn::winlayout(lua)?.get::<_, String>(1)? != "leaf";

            if is_split {
                1
            } else {
                0
            }
        },
        _ => 1,
    };

    let tablineoffset: u8 = match showtabline {
        0 => 0,
        1 => {
            let tabpages = api::list_tabpages(lua)?;

            if tabpages.len()? > 1 {
                1
            } else {
                0
            }
        },
        _ => 1,
    };

    let rows_above = screenrow - (tablineoffset + 1) as u16;

    let rows_below = lines
        - rows_above
        - (tablineoffset + statuslineoffset + cmdheight + 1) as u16;

    Ok((rows_above, rows_below))
}
