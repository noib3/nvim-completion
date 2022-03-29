use std::cmp;

use mlua::{prelude::LuaResult, Lua};
use neovim::Api;

use super::{
    details::CompletionDetails,
    hint::CompletionHint,
    menu::{self, CompletionMenu},
};
use crate::completion::{CompletionItem, Cursor};
use crate::settings::Settings;

/// `nvim-compleet`'s UI is composed of the following 3 independent pieces.
#[derive(Debug)]
pub struct Ui {
    /// A completion menu used to show all the available completion
    /// candidates.
    pub completion_menu: CompletionMenu,

    /// A hint used to show the text that would be inserted in the buffer if
    /// the current completion item was accepted.
    pub completion_hint: CompletionHint,

    /// A details pane used to show some informations about the currently
    /// selected completion item.
    pub completion_details: CompletionDetails,
}

impl Ui {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(Ui {
            completion_menu: CompletionMenu::new(api)?,
            completion_hint: CompletionHint::new(api)?,
            completion_details: CompletionDetails::new(api)?,
        })
    }
}

impl Ui {
    /// Executed on every `InsertLeave` event in attached buffers.
    pub fn cleanup(&mut self, api: &Api) -> LuaResult<()> {
        if self.completion_menu.is_visible() {
            self.completion_menu.close(api)?;

            // The details window can only be visible if the completion menu is
            // visible.
            if self.completion_details.is_visible() {
                self.completion_details.close(api)?;
            }
        }

        if self.completion_hint.is_visible() {
            self.completion_hint.erase(api)?;
        }

        Ok(())
    }

    /// Executed on every `CursorMovedI` event in attached buffers.
    pub fn update(
        &mut self,
        lua: &Lua,
        api: &Api,
        completions: &[CompletionItem],
        cursor: &Cursor,
        settings: &Settings,
    ) -> LuaResult<()> {
        // If there are no completions to display simply cleanup the UI and
        // return early.
        if completions.is_empty() {
            // TODO: reset selected & hinted indexes.
            self.cleanup(api)?;
            return Ok(());
        }

        let hint = &mut self.completion_hint;
        let menu = &mut self.completion_menu;
        let details = &mut self.completion_details;

        // Update the selected completion index if it was set.
        if let Some(old) = menu.selected_index {
            menu.selected_index = Some(cmp::min(old, completions.len() - 1));
        }

        // Let's first update the completion hint.
        if settings.ui.hint.enable && cursor.is_at_eol() {
            let index = menu.selected_index.unwrap_or(0);
            let completion = &completions[index];
            let text = &completion.text[(completion.matched_bytes as usize)..];
            hint.set(lua, api, text, cursor, index)?;
        } else if hint.is_visible() {
            hint.erase(api)?;
        }

        // Now the completion menu. The first step is to compute how big it
        // should be and where it should be placed relative to the cursor.
        let menu_position = match menu::positioning::get_position(
            api,
            completions,
            &settings.ui.menu,
        )? {
            Some(position) => position,

            // If it wasn't possible to get a position for the menu we just
            // clean the menu and the details window, then return.
            None => {
                menu.close(api)?;
                details.close(api)?;
                return Ok(());
            },
        };

        // If the menu was already visible we move it to its new position.
        if let Some(winid) = menu.winid {
            menu.shift(lua, api, &menu_position)?;

            // Reset the cursor to the first row of the window.
            // TODO: document why.
            api.win_set_cursor(winid, 1, 0)?;

            // If there'a a completion item already selected we also need to
            // update the details window.
            if let Some(index) = menu.selected_index {
                // Shifting the window resets the `cursorline` option to
                // `false`. If a completion is selected it needs to be set back
                // to `true`.
                api.win_set_option(winid, "cursorline", true)?;

                // Set the cursor row to the selected completion.
                api.win_set_cursor(winid, (index + 1).try_into().unwrap(), 0)?;

                // Update the completion details.
                let lines = completions[index].details.as_ref();
                details.update(
                    lua,
                    api,
                    lines,
                    &settings.ui.details.border,
                    menu_position.width,
                    winid,
                    &settings.ui.menu.border,
                    true,
                )?;
            }
            // If not we close it.
            else {
                details.close(api)?;
            }
        }
        // If the menu wasn't visible we create a new window.
        else {
            menu.spawn(lua, api, &menu_position, &settings.ui.menu.border)?;
        }

        // Finally, we fill the menu's buffer with the new completion items.
        menu.fill(lua, api, completions)?;

        Ok(())
    }
}
