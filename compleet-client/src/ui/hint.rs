use compleet::cursor::Cursor;
use mlua::{prelude::LuaResult, Lua};

use crate::bindings::api;

#[derive(Debug)]
pub struct CompletionHint {
    /// The namespace id associated to the completion hint.
    nsid: u32,

    /// The index of the completion currently being hinted.
    pub hinted_index: Option<usize>,
}

impl CompletionHint {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        Ok(CompletionHint {
            nsid: api::create_namespace(lua, "compleet_completion_hint")?,
            hinted_index: None,
        })
    }
}

impl CompletionHint {
    pub fn erase(&mut self, lua: &Lua) -> LuaResult<()> {
        api::buf_clear_namespace(
            lua,
            0,
            self.nsid.try_into().unwrap(),
            0,
            -1,
        )?;
        self.hinted_index = None;
        Ok(())
    }

    pub fn is_visible(&self) -> bool { self.hinted_index.is_some() }

    pub fn set(
        &mut self,
        lua: &Lua,
        text: &str,
        cursor: &Cursor,
        index: usize,
    ) -> LuaResult<()> {
        let opts = lua.create_table_with_capacity(0, 3)?;
        opts.set("id", 1)?;
        opts.set("virt_text", [[text, "CompleetHint"]])?;
        opts.set("virt_text_pos", "overlay")?;

        api::buf_set_extmark(
            lua,
            0,
            self.nsid,
            cursor.row,
            cursor.bytes,
            opts,
        )?;

        self.hinted_index = Some(index);

        Ok(())
    }
}
