use std::rc::Rc;

use mlua::{prelude::LuaResult, Lua, ToLua};

use super::floater::Floater;
use crate::bindings::api;
use crate::settings::ui::details::DetailsSettings;

#[derive(Debug)]
pub struct CompletionDetails {
    /// Number of the buffer used to show the completion details. It's created
    /// on initialization and never changes.
    bufnr: u32,

    /// Id of the floating window used to show the completion details, or
    /// `None` if the details window is currently closed.
    pub floater: Floater,
}

impl CompletionDetails {
    pub fn new(lua: &Lua, settings: &DetailsSettings) -> LuaResult<Self> {
        Ok(CompletionDetails {
            bufnr: api::create_buf(lua, false, true)?,
            floater: Floater::new(
                lua,
                &settings.border,
                vec![
                    ("FloatBorder", "CompleetDetailsBorder"),
                    ("Normal", "CompleetDetails"),
                    ("Search", "None"),
                ],
            ),
        })
    }
}

impl CompletionDetails {
    /// Fills the buffer with a list of lines.
    pub fn fill(&self, lua: &Lua, lines: &[String]) -> LuaResult<()> {
        api::buf_set_lines(lua, self.bufnr, 0, -1, false, lines)
    }

    /// Moves the window to a new location (relative to the completion menu).
    pub fn r#move(
        &self,
        lua: &Lua,
        /* pos: &WindowPosition, */
        menu_winid: u32,
    ) -> LuaResult<()> {
        // let winid = self.winid.expect("The window is open so it has an
        // id.");

        let opts = lua.create_table_from([
            ("relative", "win".to_lua(lua)?),
            ("win", menu_winid.to_lua(lua)?),
            // ("row", position.row.to_lua(lua)?);
            // ("col", position.col.to_lua(lua)?);
            // ("width", position.width.to_lua(lua)?);
            // ("height", position.height.to_lua(lua)?);
        ])?;

        // api::win_set_config(lua, winid, opts)?;

        Ok(())
    }

    // /// Opens a new window positioned relative to the completion menu.
}
