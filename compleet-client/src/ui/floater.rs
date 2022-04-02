use mlua::{prelude::LuaResult, Lua};

use crate::bindings::api;
use crate::settings::ui::border::Border;

/// A floating window.
#[derive(Debug)]
pub struct Floater {
    /// The border of the floating window.
    // border_style: LuaValue,

    /// TODO: docs
    pub id: Option<u32>,

    /// TODO: docs
    height: u16,

    /// TODO: docs,
    width: u16,

    /// TODO: docs
    winhl: String,
}

/// What the floater is positioned relative to, either the cursor or anoter
/// floater.
pub enum Relative {
    Cursor(i32, i32),
    Floater(u32, i32, i32),
}

impl Floater {
    pub fn new(
        lua: &Lua,
        border: &Border,
        hl_groups: Vec<(&'static str, &'static str)>,
    ) -> Self {
        let winhl = hl_groups
            .into_iter()
            .map(|(builtin, custom)| [builtin, custom].join(":"))
            .collect::<Vec<String>>()
            .join(",");

        Floater {
            // border,
            id: None,
            height: 0,
            width: 0,
            winhl,
        }
    }

    pub fn close(&mut self, lua: &Lua) -> LuaResult<()> {
        if let Some(winid) = self.id {
            api::win_hide(lua, winid)?;
            self.id = None;
        }
        Ok(())
    }

    pub fn is_open(&self) -> bool { self.id.is_some() }

    pub fn open(
        &mut self,
        lua: &Lua,
        bufnr: u32,
        relative: Relative,
        width: u16,
        height: u16,
    ) -> LuaResult<()> {
        let opts = lua.create_table_with_capacity(0, 10)?;

        match relative {
            Relative::Cursor(row, col) => {
                opts.set("relative", "cursor")?;
                opts.set("row", row)?;
                opts.set("col", col)?;
            },

            Relative::Floater(winid, row, col) => {
                opts.set("relative", "win")?;
                opts.set("win", winid)?;
                opts.set("row", row)?;
                opts.set("col", col)?;
            },
        };

        opts.set("width", width)?;
        opts.set("height", height)?;
        opts.set("style", "minimal")?;
        opts.set("focusable", false)?;
        opts.set("noautocmd", true)?;

        // if self.border.enable {
        //     opts.set("border", self.border.style.to_lua(lua)?)?;
        // }

        let winid = api::open_win(lua, bufnr, false, opts)?;
        api::win_set_option(lua, winid, "winhl", &*self.winhl)?;
        api::win_set_option(lua, winid, "scrolloff", 0)?;

        self.id = Some(winid);
        self.height = height;
        self.width = width;

        Ok(())
    }
}
