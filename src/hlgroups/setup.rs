use mlua::prelude::{Lua, LuaResult};
use neovim::Api;

/// Creates our highlight groups, linking them to other default groups. This
/// can be used by colorscheme plugin authors to style the UI.
pub fn setup(lua: &Lua, api: &Api) -> LuaResult<()> {
    // `CompleetMenu`
    // Used to highlight the completion menu.
    let opts = lua.create_table_from([("link", "NormalFloat")])?;
    api.set_hl(0, "CompleetMenu", opts)?;

    // `CompleetHint`
    // Used to highlight the completion hint.
    let opts = lua.create_table_from([("link", "Comment")])?;
    api.set_hl(0, "CompleetHint", opts)?;

    // `CompleetDetails`
    // Used to highlight the details window.
    let opts = lua.create_table_from([("link", "NormalFloat")])?;
    api.set_hl(0, "CompleetDetails", opts)?;

    // `CompleetMenuSelected`
    // Used to highlight the currently selected completion item.
    let opts = lua.create_table_from([("link", "PmenuSel")])?;
    api.set_hl(0, "CompleetMenuSelected", opts)?;

    // `CompleetMenuMatchingChars`
    // Used to highlight the characters where a completion item matches the
    // current completion prefix.
    let opts = lua.create_table_from([("link", "Statement")])?;
    api.set_hl(0, "CompleetMenuMatchingChars", opts)?;

    Ok(())
}
