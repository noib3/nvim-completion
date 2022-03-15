use mlua::{Function, Lua, Result, Table};

use crate::api::Api;
use crate::keymap::Keymap;

pub struct Neovim<'a> {
    /// TODO: docs
    _g: Table<'a>,

    /// TODO: docs
    pub api: Api<'a>,

    /// TODO: docs
    pub keymap: Keymap<'a>,

    /// TODO: docs
    vim: Table<'a>,
}

impl<'a> Neovim<'a> {
    pub fn new(lua: &'a Lua) -> Result<Neovim<'a>> {
        let _g = lua.globals();
        let vim = _g.get::<&str, Table>("vim")?;

        let api = Api::new(vim.clone())?;
        let keymap = Keymap::new(vim.clone())?;

        Ok(Neovim {
            _g,
            api,
            keymap,
            vim,
        })
    }
}

/// TODO: docs
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl<'a> Neovim<'a> {
    /// TODO: docs
    pub fn notify<S: AsRef<str>>(
        &self,
        msg: S,
        level: LogLevel,
        opts: Table,
    ) -> Result<()> {
        self.vim.get::<&str, Function>("notify")?.call::<_, ()>((
            msg.as_ref(),
            level as usize,
            opts,
        ))
    }

    /// TODO: docs
    pub fn print<S: AsRef<str>>(&self, msg: S) -> Result<()> {
        self._g
            .get::<&str, Function>("print")?
            .call::<_, ()>(msg.as_ref())
    }
}
