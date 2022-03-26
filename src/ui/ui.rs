use mlua::{prelude::LuaResult, Lua};
use neovim::Api;

use super::{
    details::CompletionDetails,
    hint::CompletionHint,
    menu::CompletionMenu,
    QueuedUpdates,
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

    /// TODO: docs
    pub queued_updates: QueuedUpdates,
}

impl Ui {
    pub fn new(api: &Api) -> LuaResult<Self> {
        Ok(Ui {
            completion_menu: CompletionMenu::new(api)?,
            completion_hint: CompletionHint::new(api)?,
            completion_details: CompletionDetails::new(api)?,
            queued_updates: QueuedUpdates::new(),
        })
    }
}

impl Ui {
    /// Executed on every `InsertLeft` event of attached buffers.
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

    /// Executed on every `CursorMovedI` event of attached buffers.
    pub fn update(
        &mut self,
        lua: &Lua,
        api: &Api,
        completions: &[CompletionItem],
        cursor: &Cursor,
        settings: &Settings,
    ) -> LuaResult<()> {
        let menu = &mut self.completion_menu;
        let details = &mut self.completion_details;
        let hint = &mut self.completion_hint;
        let updates = &mut self.queued_updates;

        // Update the completion menu and completion details.
        match (menu.winid, updates.menu_position.as_ref()) {
            (Some(winid), Some(position)) => {
                menu.shift(lua, api, position)?;
                menu.fill(lua, api, completions)?;

                // Reset the cursor to the first row of the window.
                api.win_set_cursor(winid, 1, 0)?;

                if let Some(index) = menu.selected_index {
                    // Shifting the window resets the `cursorline` option to
                    // `false`. If a completion is selected it needs to be set
                    // back to `true`.
                    api.win_set_option(winid, "cursorline", true)?;

                    // Set the cursor row to the selected completion.
                    api.win_set_cursor(
                        winid,
                        (index + 1).try_into().unwrap(),
                        0,
                    )?;

                    // Update the completion details.
                    let lines = completions[index].details.as_ref();
                    details.update(
                        lua,
                        api,
                        lines,
                        &settings.ui.details.border,
                        position.width,
                        winid,
                        &settings.ui.menu.border,
                        true,
                    )?;
                } else {
                    details.close(&api)?;
                }
            },

            (None, Some(position)) => {
                menu.spawn(lua, api, position, &settings.ui.menu.border)?;
                menu.fill(lua, api, completions)?;
            },

            (Some(_), None) => {
                menu.close(api)?;
                details.close(api)?;
            },

            (None, None) => {},
        }

        // Update the completion hint.
        let comp_with_i = updates.hinted_index.map(|i| (&completions[i], i));
        hint.update(lua, api, comp_with_i, cursor)?;

        // After we've consumed all the instructions we reset them.
        updates.reset();

        Ok(())
    }
}
