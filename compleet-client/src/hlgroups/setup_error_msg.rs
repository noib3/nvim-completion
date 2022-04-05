use mlua::prelude::{Lua, LuaResult};

use crate::bindings::api;
use crate::constants::*;

/// Sets the highlight groups used in the warning and error messages.
pub fn setup_error_msg(lua: &Lua) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    // Used to highlight the `[nvim-compleet]` tag of all error messages.
    opts.set("link", "ErrorMsg")?;
    api::set_hl(lua, 0, HLGROUP_ERROR_MSG_TAG, opts.clone())?;

    // Used to highlight the `[nvim-compleet]` tag of all warning messages.
    opts.set("link", "WarningMsg")?;
    api::set_hl(lua, 0, HLGROUP_WARNING_MSG_TAG, opts.clone())?;

    // Used to highlight the path of the config option that caused a
    // deserialization error.
    opts.set("link", "Statement")?;
    api::set_hl(lua, 0, HLGROUP_OPTION_PATH, opts.clone())?;

    // Used to highlight any field of the error message enclosed by backticks.
    opts.set("link", "Special")?;
    api::set_hl(lua, 0, HLGROUP_MSG_FIELD, opts)?;

    Ok(())
}
