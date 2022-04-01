use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;

/// Sets up the highlight groups, linking them to pre-existing defaults. These
/// can be used by colorscheme plugin authors to style the UI.
pub fn setup(lua: &Lua) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    // `CompleetMenu`
    // Used to highlight the completion menu.
    opts.set("link", "NormalFloat")?;
    api::set_hl(lua, 0, "CompleetMenu", opts.clone())?;

    // `CompleetMenuSelected`
    // Used to highlight the currently selected completion item.
    opts.set("link", "PmenuSel")?;
    api::set_hl(lua, 0, "CompleetMenuSelected", opts.clone())?;

    // `CompleetMenuMatchingChars`
    // Used to highlight the characters where a completion item matches the
    // current completion prefix.
    opts.set("link", "Statement")?;
    api::set_hl(lua, 0, "CompleetMenuMatchingChars", opts.clone())?;

    // `CompleetMenuBorder`
    // Used to highlight the border of the completion menu.
    opts.set("link", "FloatBorder")?;
    api::set_hl(lua, 0, "CompleetMenuBorder", opts.clone())?;

    // `CompleetDetails`
    // Used to highlight the details window.
    opts.set("link", "NormalFloat")?;
    api::set_hl(lua, 0, "CompleetDetails", opts.clone())?;

    // `CompleetDetailsBorder`
    // Used to highlight the border of the details window.
    opts.set("link", "FloatBorder")?;
    api::set_hl(lua, 0, "CompleetDetailsBorder", opts.clone())?;

    // `CompleetHint`
    // Used to highlight the completion hint.
    opts.set("link", "Comment")?;
    api::set_hl(lua, 0, "CompleetHint", opts.clone())?;

    Ok(())
}
