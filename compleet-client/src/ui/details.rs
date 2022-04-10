use std::cmp;

use mlua::{prelude::LuaResult, Lua};
use sources::completion::CompletionItem;

use super::floater::{Floater, RelativeTo};
use crate::bindings::api;
use crate::constants::hlgroups::ui;
use crate::settings::ui::details::DetailsSettings;

#[derive(Debug, Default)]
pub struct CompletionDetails {
    /// Number of the buffer used to show the completion details. It's created
    /// once in `CompletionDetails::new` and never changes.
    bufnr: u32,

    /// Floating window used to show the completion details.
    pub floater: Floater,
}

impl CompletionDetails {
    pub fn new(lua: &Lua, settings: &DetailsSettings) -> LuaResult<Self> {
        let bufnr = api::create_buf(lua, false, true)?;
        Ok(CompletionDetails {
            bufnr,
            floater: Floater::new(
                lua,
                bufnr,
                &settings.border,
                vec![
                    ("FloatBorder", ui::DETAILS_BORDER),
                    ("Normal", ui::DETAILS),
                    ("Search", "None"),
                ],
            )?,
        })
    }
}

impl CompletionDetails {
    /// Fills the buffer with a list of lines.
    pub fn fill(&self, lua: &Lua, lines: Vec<String>) -> LuaResult<()> {
        api::buf_set_lines(lua, self.bufnr, 0, -1, false, lines)
    }

    /// Updates both the contents and the position of the details window.
    pub fn update(
        &mut self,
        lua: &Lua,
        completion: Option<&CompletionItem>,
        menu: &Floater,
        force_reopen: bool,
    ) -> LuaResult<()> {
        // If there's not a new completion or the completion has no details to
        // display we simply close the floater and return.
        if completion.and_then(|c| c.details.as_ref()).is_none() {
            self.floater.close(lua)?;
            return Ok(());
        }

        // The lines to fill the buffer with.
        let lines: &[String] = completion
            .expect("Already checked `None` variant")
            .details
            .as_ref()
            .expect("Already checked `None` variant");

        // The window id of the completion menu's floating window.
        let menu_winid = menu.id.expect("The menu is open");

        let (row, col, height, width) =
            match self::find_position(lua, lines, menu, &self.floater)? {
                Some(tuple) => tuple,

                // If we couldn't find a way to position the new details window
                // we close the current one and return.
                None => {
                    self.floater.close(lua)?;
                    return Ok(());
                },
            };

        let position = RelativeTo::Floater(menu_winid, row, col);

        // If the floater was already open we should just move it to the new
        // position. Unfortunately because of an upstream bug
        // (https://github.com/neovim/neovim/issues/17853) this doesn't always
        // work, in which case we need to close and reopen it.
        if self.floater.is_open() {
            if !force_reopen {
                self.floater.r#move(lua, position, height, width)?;
            } else {
                self.floater.close(lua)?;
                self.floater.open(lua, position, height, width)?;
            }
        }
        // If the floater was closed we open it.
        else {
            self.floater.open(lua, position, height, width)?;
        }

        // Lastly, fill the buffer with the new lines.
        self.fill(lua, lines.to_vec())?;

        Ok(())
    }
}

/// Tries to find a way to position the details window relative to the
/// completion menu. In other words, it decides where to place a rectangle (the
/// details window) relative to another rectangle (the completion menu), with
/// the constraint of having to fit inside an outer, bigger rectangle (the
/// terminal).
fn find_position(
    lua: &Lua,
    lines: &[String],
    menu: &Floater,
    details: &Floater,
) -> LuaResult<Option<(i32, i32, u16, u16)>> {
    // TODO: add a max.
    let height = lines.len() as u16;

    let width = lines
        .iter()
        // The number of bytes isn't the best indicator for how many terminal
        // columns a line will actually take up, but that problem isn't well
        // defined anyways so this will suffice.
        .map(|line| line.len())
        .max()
        // TODO: a max width of 79 is arbitrary.
        .map(|longest| cmp::min(longest, 79))
        .expect("There's at least one line") as u16;

    // The total width of the details window, also counting the left and right
    // edges of its border.
    let width_with_borders = width
        + if details.border_edges[2] { 1 } else { 0 }
        + if details.border_edges[3] { 1 } else { 0 };

    // The number of columns before and after the completion menu.
    let (cols_before, cols_after) = menu.cols_before_after(lua)?;

    // TODO: better logic.
    //
    // Horizontal policy.
    //
    // First we try to display the details to the right of the completion menu.
    let col = if width_with_borders <= cols_after {
        (menu.width + if menu.border_edges[3] { 1 } else { 0 }) as i32
    }
    // If there's not enough space we try to display it to the left.
    else if width_with_borders <= cols_before {
        -(width as i32)
            - if menu.border_edges[2] { 1 } else { 0 }
            - if details.border_edges[3] { 1 } else { 0 }
            // TODO: why does the left border of the details window need to be
            // counted?
            - if details.border_edges[2] { 1 } else { 0 }
    }
    // If that also fails we give up and return `None`.
    else {
        return Ok(None);
    };

    // TODO: better logic.
    //
    // Vertical policy.
    //
    // The top edge of the details window always lines up with the top edge of
    // the completion menu.
    let row = if menu.border_edges[0] { -1 } else { 0 };

    Ok(Some((row, col, height, width)))
}
