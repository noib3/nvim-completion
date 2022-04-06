use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;
use crate::constants::hlgroups::ui;

/// Sets up the highlight groups, linking them to pre-existing defaults. These
/// can be used by colorscheme plugin authors to style the UI.
pub fn setup(lua: &Lua) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    // Used to highlight the completion menu.
    opts.set("link", "NormalFloat")?;
    api::set_hl(lua, 0, ui::MENU, opts.clone())?;

    // Used to highlight the currently selected completion item.
    opts.set("link", "PmenuSel")?;
    api::set_hl(lua, 0, ui::MENU_SELECTED, opts.clone())?;

    // Used to highlight the characters where a completion item matches the
    // current completion prefix.
    opts.set("link", "Statement")?;
    api::set_hl(lua, 0, ui::MENU_MATCHING, opts.clone())?;

    // Used to highlight the border of the completion menu.
    opts.set("link", "FloatBorder")?;
    api::set_hl(lua, 0, ui::MENU_BORDER, opts.clone())?;

    // Used to highlight the details window.
    opts.set("link", "NormalFloat")?;
    api::set_hl(lua, 0, ui::DETAILS, opts.clone())?;

    // Used to highlight the border of the details window.
    opts.set("link", "FloatBorder")?;
    api::set_hl(lua, 0, ui::DETAILS_BORDER, opts.clone())?;

    // Used to highlight the completion hint.
    opts.set("link", "Comment")?;
    api::set_hl(lua, 0, ui::HINT, opts.clone())?;

    Ok(())
}
