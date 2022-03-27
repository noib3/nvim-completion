use mlua::prelude::{Lua, LuaResult};
use neovim::Api;

/// Sets up the highlight groups used in the error message displayed if any
/// option passed to the `setup` function is invalid.
pub fn setup_error_msg(lua: &Lua, api: &Api) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    // `CompleetErrorMsgTag` Used to highlight the `[nvim-compleet]` part of
    // any error message.
    opts.set("link", "ErrorMsg")?;
    api.set_hl(0, "CompleetErrorMsgTag", opts.clone())?;

    // `CompleetErrorMsgOptionPath`
    // Used to highlight the path of the option that caused the error message.
    opts.set("link", "Statement")?;
    api.set_hl(0, "CompleetErrorMsgOptionPath", opts.clone())?;

    // `CompleetErrorMsgField`
    // Used to highlight any field of the error message enclosed by backticks
    // ('``), except for the option path.
    opts.set("link", "Special")?;
    api.set_hl(0, "CompleetErrorMsgField", opts)?;

    Ok(())
}
