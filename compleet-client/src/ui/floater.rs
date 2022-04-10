use mlua::prelude::{Lua, LuaRegistryKey, LuaResult, LuaValue};

use crate::bindings::api;
use crate::settings::ui::border::Border;

/// Abstracts Neovim's floating windows (see `:h api-floatwin` for details).
#[derive(Debug, Default)]
pub struct Floater {
    /// An array of the form `[top_edge, bottom_edge, left_edge, right_edge]`
    /// where each element is `true` if that edge of the border is present.
    pub border_edges: [bool; 4],

    /// The registry key of the Lua table defining the style of the border.
    /// Its general shape is described in `:h nvim_open_win`. Set to `None` if
    /// the floater doesn't have a border.
    border_style_key: Option<LuaRegistryKey>,

    /// The number of the buffer this floating window should contain.
    bufnr: u32,

    /// The window id of the floater, or `None` if it's currently closed.
    pub id: Option<u32>,

    #[allow(dead_code)]
    /// The height of the floater in terminal rows, **not** including the
    /// top and bottom edges of the border (if present).
    pub height: u16,

    /// The width of the floater in terminal columns, **not** including the
    /// left and right edges of the border (if present).
    pub width: u16,

    /// Window-local highlights. See `:h winhl` for details.
    winhl: String,
}

/// What the floater is positioned relative to, either the cursor or anoter
/// floater.
pub enum RelativeTo {
    Cursor(i32, i32),
    Floater(u32, i32, i32),
}

impl Floater {
    pub fn new(
        lua: &Lua,
        bufnr: u32,
        border: &Border,
        hl_groups: Vec<(&'static str, &'static str)>,
    ) -> LuaResult<Self> {
        let (border_style_key, border_edges) = if border.enable {
            (None, [false; 4])
        } else {
            let style = border.style.to_lua(lua)?;

            let edges = [
                border.style.has_top_edge(),
                border.style.has_bottom_edge(),
                border.style.has_left_edge(),
                border.style.has_right_edge(),
            ];

            (Some(lua.create_registry_value(style)?), edges)
        };

        let winhl = hl_groups
            .into_iter()
            .map(|(builtin, custom)| [builtin, custom].join(":"))
            .collect::<Vec<String>>()
            .join(",");

        Ok(Floater {
            border_edges,
            border_style_key,
            bufnr,
            id: None,
            height: 0,
            width: 0,
            winhl,
        })
    }

    /// Closes the floater, setting its `id` field back to `None`.
    pub fn close(&mut self, lua: &Lua) -> LuaResult<()> {
        if let Some(winid) = self.id {
            api::win_hide(lua, winid)?;
            self.id = None;
        }
        Ok(())
    }

    /// Returns the number of columns before and after the floater, also
    /// considering the left and right edges of its border. Should only be
    /// called if the floater is open.
    pub fn cols_before_after(&self, lua: &Lua) -> LuaResult<(u16, u16)> {
        let cols_total = api::get_option::<u16>(lua, "columns")?;

        let winid = self.id.expect("The floater is open so it has an id");

        // BUG: the `col` of `win_get_position` is sometimes bigger that the
        // total number of columns, causing the following subtractions to
        // overflow.
        //
        // TODO: Open an issue upstream.
        let mut cols_before = api::win_get_position(lua, winid)?.1;

        let cols_after = cols_total
            - cols_before
            - self.width
            - if self.border_edges[3] { 1 } else { 0 };

        // If the left edge of the border is set there's one less available
        // column before the floater.
        if cols_before > 0 && self.border_edges[2] {
            cols_before -= 1;
        }

        Ok((cols_before, cols_after))
    }

    /// Whether the floater is currently open.
    pub fn is_open(&self) -> bool {
        self.id.is_some()
    }

    /// Opens a new floater
    pub fn open(
        &mut self,
        lua: &Lua,
        position: RelativeTo,
        height: u16,
        width: u16,
    ) -> LuaResult<()> {
        let opts = lua.create_table_with_capacity(0, 10)?;

        match position {
            RelativeTo::Cursor(row, col) => {
                opts.set("relative", "cursor")?;
                opts.set("row", row)?;
                opts.set("col", col)?;
            },

            RelativeTo::Floater(winid, row, col) => {
                opts.set("relative", "win")?;
                opts.set("win", winid)?;
                opts.set("row", row)?;
                opts.set("col", col)?;
            },
        };

        opts.set("height", height)?;
        opts.set("width", width)?;
        opts.set("style", "minimal")?;
        opts.set("focusable", false)?;
        opts.set("noautocmd", true)?;

        if let Some(key) = &self.border_style_key {
            // Retrieve the Lua table defining the border style from its
            // registry key.
            opts.set("border", lua.registry_value::<LuaValue>(key)?)?;
        }

        let winid = api::open_win(lua, self.bufnr, false, opts)?;
        api::win_set_option(lua, winid, "winhl", &*self.winhl)?;
        api::win_set_option(lua, winid, "scrolloff", 0)?;

        self.id = Some(winid);
        self.height = height;
        self.width = width;

        Ok(())
    }

    /// Moves the floater to a new position. Should only be called if the
    /// floater is already open. If not use `Floater::open` instead.
    pub fn r#move(
        &mut self,
        lua: &Lua,
        position: RelativeTo,
        width: u16,
        height: u16,
    ) -> LuaResult<()> {
        let winid = self.id.expect("The floater is open so it has an id");

        let opts = lua.create_table_with_capacity(0, 6)?;

        match position {
            RelativeTo::Cursor(row, col) => {
                opts.set("relative", "cursor")?;
                opts.set("row", row)?;
                opts.set("col", col)?;
            },

            RelativeTo::Floater(winid, row, col) => {
                opts.set("relative", "win")?;
                opts.set("win", winid)?;
                opts.set("row", row)?;
                opts.set("col", col)?;
            },
        };

        opts.set("height", height)?;
        opts.set("width", width)?;

        api::win_set_config(lua, winid, opts)?;
        self.height = height;
        self.width = width;

        Ok(())
    }
}
