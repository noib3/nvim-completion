use mlua::prelude::{Lua, LuaResult};
use neovim::Api;

/// Creates our highlight groups, linking them to other default groups. This
/// can be used by colorscheme plugin authors to style the UI.
pub fn setup(lua: &Lua, api: &Api) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 1)?;

    // `CompleetMenu`
    // Used to highlight the completion menu.
    opts.set("link", "NormalFloat")?;
    api.set_hl(0, "CompleetMenu", opts.clone())?;

    // `CompleetMenuSelected`
    // Used to highlight the currently selected completion item.
    opts.set("link", "PmenuSel")?;
    api.set_hl(0, "CompleetMenuSelected", opts.clone())?;

    // `CompleetMenuMatchingChars`
    // Used to highlight the characters where a completion item matches the
    // current completion prefix.
    opts.set("link", "Statement")?;
    api.set_hl(0, "CompleetMenuMatchingChars", opts.clone())?;

    // `CompleetMenuBorder`
    // Used to highlight the border of the completion menu.
    opts.set("link", "FloatBorder")?;
    api.set_hl(0, "CompleetMenuBorder", opts.clone())?;

    // `CompleetDetails`
    // Used to highlight the details window.
    opts.set("link", "NormalFloat")?;
    api.set_hl(0, "CompleetDetails", opts.clone())?;

    // `CompleetDetailsBorder`
    // Used to highlight the border of the details window.
    opts.set("link", "FloatBorder")?;
    api.set_hl(0, "CompleetDetailsBorder", opts.clone())?;

    // `CompleetHint`
    // Used to highlight the completion hint.
    opts.set("link", "Comment")?;
    api.set_hl(0, "CompleetHint", opts.clone())?;

    Ok(())
}
