use std::{fmt, ops::Range};

use mlua::prelude::{FromLua, Lua, LuaResult};

use crate::api;

/// The function signature of the `on_bytes` Lua callback passed to
/// `vim.api.nvim_buf_attach`.
pub type OnBytesSignature =
    (String, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);

// pub type OnBytesHook = Box<
//     dyn 'static
//         + Send
//         + for<'lua> Fn(&'lua Lua, OnBytesSignature) -> mlua::Result<()>,
// >;

pub type OnBytesHook = LuaFn<OnBytesSignature, Option<bool>>;

pub type LuaFn<A, R> =
    Box<dyn 'static + for<'lua> Fn(&'lua Lua, A) -> mlua::Result<R>>;

pub type LuaFnMut<A, R> =
    Box<dyn 'static + for<'lua> FnMut(&'lua Lua, A) -> mlua::Result<R>>;

// pub type LuaFn = Box<
//     dyn 'static
//         + Send
//         + for<'lua> Fn(&'lua Lua, dyn ToLuaMulti<'lua>) -> mlua::Result<()>,
// >;

/// TODO: docs
pub enum LineSelect {
    All,
    Range(Range<u16>),
    Single(u16),
    FromEnd(Range<u16>),
}

/// TODO: docs
#[derive(Debug, Default, PartialEq)]
pub struct Buffer {
    pub bufnr: u32,
    pub filepath: String,
    pub filetype: String,
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: print filename
        // write!(f, "{}", if !self.filepath)
        write!(f, "{}", self.bufnr)
    }
}

impl Buffer {
    /// Returns the current buffer.
    pub fn get_current(lua: &Lua) -> LuaResult<Self> {
        let bufnr = api::get_current_buf(lua)?;
        let filepath = api::buf_get_name(lua, bufnr)?;
        let filetype = api::buf_get_option::<String>(lua, bufnr, "filetype")?;

        Ok(Self { bufnr: bufnr.into(), filepath, filetype })
    }

    /// Creates and returns a new buffer.
    pub fn new(
        lua: &Lua,
        is_listed: bool,
        is_scratch: bool,
    ) -> LuaResult<Self> {
        let bufnr = api::create_buf(lua, is_listed, is_scratch)?.into();

        Ok(Self { bufnr, ..Default::default() })
    }

    /// TODO: docs
    pub fn set_lines<Line: Into<String>, Lines: IntoIterator<Item = Line>>(
        &self,
        lua: &Lua,
        selector: LineSelect,
        lns: Lines,
    ) -> LuaResult<()> {
        let lines = lns.into_iter().map(|l| l.into()).collect::<Vec<String>>();

        use LineSelect::*;
        let (start, end) = match selector {
            All => (0, -1),
            Range(range) => (range.start as i32, range.end as i32),
            Single(index) => (index as i32, index as i32),
            FromEnd(_range) => todo!(),
        };

        api::buf_set_lines(lua, self.bufnr, start, end, false, lines)
    }

    /// TODO: docs
    pub fn get_option<'lua, V: FromLua<'lua>>(
        &self,
        lua: &'lua Lua,
        name: &str,
    ) -> LuaResult<V> {
        api::buf_get_option(lua, self.bufnr, name)
    }

    pub fn is_modifiable(&self, lua: &Lua) -> mlua::Result<bool> {
        self.get_option(lua, "modifiable")
    }

    /// TODO: docs
    pub fn attach_on_bytes(
        &self,
        lua: &Lua,
        hook: OnBytesHook,
    ) -> LuaResult<bool> {
        let callback = lua.create_function(hook)?;
        let opts = lua.create_table_from([("on_bytes", callback)])?;
        api::buf_attach(lua, self.bufnr, false, opts)
    }
}
