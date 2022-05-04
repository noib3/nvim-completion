use bindings::api;
use mlua::{prelude::LuaResult, Lua, ToLua};
use sources::prelude::{CompletionItem, Cursor};

use crate::constants::hlgroups::ui;

#[derive(Debug, Default)]
pub struct CompletionHint {
    /// Whether the completion hint is currenty visible.
    pub is_visible: bool,

    /// The namespace id associated to the completion hint.
    nsid: u16,
}

impl CompletionHint {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        Ok(CompletionHint {
            is_visible: false,
            nsid: api::create_namespace(
                lua,
                "compleet_completion_hint".into(),
            )?,
        })
    }
}

impl CompletionHint {
    pub fn erase(&mut self, lua: &Lua) -> LuaResult<()> {
        let nsid = i32::try_from(self.nsid).unwrap();
        api::buf_clear_namespace(lua, 0, nsid, 0, -1)?;
        self.is_visible = false;
        Ok(())
    }

    pub fn set(
        &mut self,
        lua: &Lua,
        completion: &CompletionItem,
        cursor: &Cursor,
        matched_bytes: usize,
    ) -> LuaResult<()> {
        let text = &completion.text[matched_bytes..];

        let opts = lua.create_table_from([
            ("id", 1u8.to_lua(lua)?),
            ("virt_text", [[text, ui::HINT]].to_lua(lua)?),
            ("virt_text_pos", "overlay".to_lua(lua)?),
        ])?;

        api::buf_set_extmark(
            lua,
            0,
            self.nsid,
            cursor.row,
            cursor.bytes.try_into().unwrap(),
            opts,
        )?;

        self.is_visible = true;

        Ok(())
    }
}
