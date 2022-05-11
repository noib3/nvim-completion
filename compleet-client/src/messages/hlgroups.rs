use bindings::api;
use mlua::Lua;

/// Highlights the prefix tag of error messages.
pub const TAG_ERROR: &str = "CompleetErrorMsgTag";

/// Highlights the prefix tag of warning messages.
pub const TAG_WARNING: &str = "CompleetWarningMsgTag";

/// Highlights the prefix tag of info messages.
pub const TAG_INFOS: &str = "CompleetInfoMsgTag";

/// Highlights the path of the configuration option that caused a
/// deserialization error.
pub const BAD_CONFIG_PATH: &str = "CompleetOptionPath";

/// Used to highlight any field of the error message enclosed by backticks.
pub const MSG_FIELD: &str = "CompleetMsgField";

pub fn setup(lua: &Lua) -> mlua::Result<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    opts.set("link", "ErrorMsg")?;
    api::set_hl(lua, 0, self::TAG_ERROR, opts.clone())?;

    opts.set("link", "WarningMsg")?;
    api::set_hl(lua, 0, self::TAG_WARNING, opts.clone())?;

    opts.set("link", "Question")?;
    api::set_hl(lua, 0, self::TAG_INFOS, opts.clone())?;

    opts.set("link", "Statement")?;
    api::set_hl(lua, 0, self::BAD_CONFIG_PATH, opts.clone())?;

    opts.set("link", "Special")?;
    api::set_hl(lua, 0, self::MSG_FIELD, opts)?;

    Ok(())
}
