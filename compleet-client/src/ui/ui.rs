use mlua::{prelude::LuaResult, Lua};

use super::details::CompletionDetails;
use super::hint::CompletionHint;
use super::menu::CompletionMenu;
use crate::settings::ui::UiSettings;

/// The client UI is composed of the following 3 independent pieces.
#[derive(Debug)]
pub struct Ui {
    /// A hint used to show the text that would be inserted in the buffer if
    /// a completion was accepted.
    pub hint: CompletionHint,

    /// The menu used to show all the available completion items.
    pub menu: CompletionMenu,

    /// Used to show additional information about the currently selected
    /// completion.
    pub details: CompletionDetails,
}

impl Ui {
    pub fn new(lua: &Lua, settings: &UiSettings) -> LuaResult<Self> {
        Ok(Ui {
            hint: CompletionHint::new(lua)?,
            menu: CompletionMenu::new(lua, &settings.menu)?,
            details: CompletionDetails::new(lua, &settings.details)?,
        })
    }
}
