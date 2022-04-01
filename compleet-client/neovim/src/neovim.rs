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

impl<'a> Neovim<'a> {
    /// TODO: docs
    pub fn inspect(&self, t: Table) -> Result<String> {
        self.vim
            .get::<&str, Table>("inspect")?
            .get::<&str, Function>("inspect")?
            .call::<_, String>(t)
    }

    /// TODO: docs
    pub fn print<S: std::fmt::Display>(&self, msg: S) -> Result<()> {
        self._g
            .get::<&str, Function>("print")?
            .call::<_, ()>(msg.to_string())
    }

    /// TODO: docs
    pub fn schedule(&self, callback: Function) -> Result<()> {
        self.vim
            .get::<&str, Function>("schedule")?
            .call::<_, ()>(callback)
    }
}
