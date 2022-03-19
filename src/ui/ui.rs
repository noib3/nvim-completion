use mlua::{prelude::LuaResult, Lua};
use neovim::Api;

use super::{
    details::CompletionDetails, hint::CompletionHint, menu::CompletionMenu,
    DrawInstructions,
};
use crate::completion::{CompletionItem, Cursor};

/// `nvim-compleet`'s UI is composed of the following 3 independent pieces.
#[derive(Debug)]
pub struct UI {
    /// A completion menu used to show all the available completion candidates.
    pub completion_menu: CompletionMenu,

    /// A hint used to show the text that would be inserted in the buffer if
    /// the current completion item was accepted.
    pub completion_hint: CompletionHint,

    /// A details pane used to show some informations about the currently
    /// selected completion item.
    pub completion_details: CompletionDetails,

    pub queued_updates: DrawInstructions,
}

impl UI {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(UI {
            completion_menu: CompletionMenu::new(api)?,
            completion_hint: CompletionHint::new(api)?,
            completion_details: CompletionDetails::new(api)?,
            queued_updates: DrawInstructions::new(),
        })
    }
}

impl UI {
    /// Executed on every `InsertLeft` event.
    pub fn cleanup(&mut self, api: &Api) -> LuaResult<()> {
        if self.completion_menu.is_visible() {
            self.completion_menu.close(api)?;

            // The details pane can only be visible if the completion menu is
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

    /// Executed on every `CursorMovedI` event.
    pub fn update(
        &mut self,
        lua: &Lua,
        api: &Api,
        cursor: &Cursor,
        completions: &[CompletionItem],
    ) -> LuaResult<()> {
        let menu = &mut self.completion_menu;
        let hint = &mut self.completion_hint;
        let details = &mut self.completion_details;
        let updates = &mut self.queued_updates;

        // Update the completion menu.
        match (menu.winid, updates.menu_position.as_ref()) {
            (Some(winid), Some(position)) => {
                menu.shift(lua, api, position)?;
                menu.fill(lua, api, completions)?;

                // Reset the cursor to the top of the buffer.
                api.win_set_cursor(winid, 1u32, 0)?;

                // Display the selected completion.
                if let Some(index) = menu.selected_index {
                    api.win_set_cursor(
                        winid,
                        (index + 1).try_into().unwrap(),
                        0,
                    )?;

                    // Shifting the window resets the `cursorline` option to
                    // false.
                    api.win_set_option(winid, "cursorline", true)?;
                }
            },

            (None, Some(position)) => {
                menu.spawn(lua, api, position)?;
                menu.fill(lua, api, completions)?;
            },

            (Some(_), None) => {
                menu.close(api)?;
                if details.is_visible() {
                    details.close(api)?;
                }
            },

            (None, None) => {},
        }

        // Update the completion hint.
        match (hint.is_visible(), updates.hinted_index) {
            (_, Some(index)) => {
                let completion = &completions[index];
                let text = &completion.text[completion.matched_prefix_len..];
                hint.set(lua, api, text, cursor.row, cursor.bytes, index)?;
            },

            (true, None) => hint.erase(api)?,

            (false, None) => {},
        }

        // After we've consumed all the instructions we reset them.
        updates.reset();

        Ok(())
    }
}
